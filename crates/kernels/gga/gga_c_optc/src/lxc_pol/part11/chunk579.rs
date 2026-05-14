//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 579/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk579<F: Float>(t4884: F, t818: F, t2520: F, t4868: F, t2444: F, t3640: F, t4770: F, t4774: F, t4778: F, t232: F, t4818: F, t799: F, t2373: F, t4854: F, t837: F, t2454: F, t2461: F, t3687: F, t4787: F, t4794: F, t4800: F, t4802: F, t4806: F, t4809: F, t4812: F) -> (F, F, F, F, F, F, F, F) {
    let t4885 = t4884 * t818;
    let t4888 = t4868 * t2520;
    let t4895 = t2444 + 0.11872222222222222222e-1 * t3640 - 0.11872222222222222222e-1 * t4770 + 0.35616666666666666666e-1 * t4774 - 0.17808333333333333333e-1 * t4778;
    let t4897 = 0.62182e-1 * t4895 * t232;
    let t4898 = t4818 * t799;
    let t4900 = 2.0 * t2373 * t4898;
    let t4904 = t4854 * t837;
    let t4919 = -0.1294625e1 * t4787 + 0.258925e1 * t4794 + t2454 + 0.20128333333333333334e0 * t3640 - 0.20128333333333333333e0 * t4770 + 0.60385e0 * t4774 - 0.301925e0 * t4778 + 0.82524375e-1 * t4800 + 0.16504875e0 * t4802 + t2461 + 0.11038e0 * t3687 - 0.27595e-1 * t4806 + 0.16557e0 * t4809 - 0.82785e-1 * t4812;
    (t4885, t4888, t4895, t4897, t4898, t4900, t4904, t4919)
}
