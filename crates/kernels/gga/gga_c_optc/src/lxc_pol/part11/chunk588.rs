//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 588/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk588<F: Float>(t2418: F, t4818: F, t2416: F, t2258: F, t2281: F, t3640: F, t3687: F, t4770: F, t4774: F, t4778: F, t4806: F, t4809: F, t4812: F) -> (F, F, F) {
    let t4819 = t4818 * t2418;
    let t4821 = F::cast_from(0.16081824322151104822e2_f64) * t2416 * t4819;
    let t4831 = t2258 + F::cast_from(0.12925555555555555555e1_f64) * t3640 - F::cast_from(0.12925555555555555555e1_f64) * t4770 + F::cast_from(0.38776666666666666666e1_f64) * t4774 - F::cast_from(0.19388333333333333333e1_f64) * t4778 + t2281 + F::cast_from(0.1642e-2_f64) * t3687 - F::cast_from(0.4105e-3_f64) * t4806 + F::cast_from(0.2463e-2_f64) * t4809 - F::cast_from(0.12315e-2_f64) * t4812;
    (t4819, t4821, t4831)
}
