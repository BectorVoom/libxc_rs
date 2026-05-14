//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 574/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk574<F: Float>(t2418: F, t4818: F, t2416: F, t2258: F, t2281: F, t3640: F, t3687: F, t4770: F, t4774: F, t4778: F, t4806: F, t4809: F, t4812: F, t1426: F) -> (F, F, F, F) {
    let t4819 = t4818 * t2418;
    let t4821 = 0.16081824322151104822e2 * t2416 * t4819;
    let t4831 = t2258 + 0.12925555555555555555e1 * t3640 - 0.12925555555555555555e1 * t4770 + 0.38776666666666666666e1 * t4774 - 0.19388333333333333333e1 * t4778 + t2281 + 0.1642e-2 * t3687 - 0.4105e-3 * t4806 + 0.2463e-2 * t4809 - 0.12315e-2 * t4812;
    let t4835 = t1426 * t1426;
    (t4819, t4821, t4831, t4835)
}
