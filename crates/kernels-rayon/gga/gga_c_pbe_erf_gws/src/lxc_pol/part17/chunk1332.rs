//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1332/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1332(t13796: f64, t14724: f64, t2352: f64, t343: f64, t3989: f64, t13972: f64, t14684: f64, t14767: f64, t2397: f64, t1134: f64, t13776: f64, t2410: f64, t50956: f64) -> (f64, f64, f64, f64) {
    let t54461 = t3989 * t13796 * t14724 * t343 * t2352;
    let t54463 = t13972 * t14684;
    let t54464 = 7.0_f64 / 1152.0_f64 * t54463;
    let t54465 = t14767 * t2397;
    let t54473 = t13776 * t50956 * t1134 * t2410;
    (t54461, t54464, t54465, t54473)
}
