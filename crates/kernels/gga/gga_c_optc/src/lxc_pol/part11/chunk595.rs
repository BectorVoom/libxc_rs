//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 595/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk595<F: Float>(t2373: F, t4898: F, t4854: F, t837: F, t2454: F, t2461: F, t3640: F, t3687: F, t4770: F, t4774: F, t4778: F, t4787: F, t4794: F, t4800: F, t4802: F, t4806: F, t4809: F, t4812: F) -> (F, F, F) {
    let t4900 = F::new(2.0) * t2373 * t4898;
    let t4904 = t4854 * t837;
    let t4919 = -F::new(0.1294625e1) * t4787 + F::new(0.258925e1) * t4794 + t2454 + F::cast_from(0.20128333333333333334e0_f64) * t3640 - F::cast_from(0.20128333333333333333e0_f64) * t4770 + F::new(0.60385e0) * t4774 - F::new(0.301925e0) * t4778 + F::new(0.82524375e-1) * t4800 + F::new(0.16504875e0) * t4802 + t2461 + F::new(0.11038e0) * t3687 - F::new(0.27595e-1) * t4806 + F::new(0.16557e0) * t4809 - F::new(0.82785e-1) * t4812;
    (t4900, t4904, t4919)
}
