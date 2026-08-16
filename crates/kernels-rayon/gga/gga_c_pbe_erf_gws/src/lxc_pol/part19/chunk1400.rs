//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1400/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1400(t14918: f64, t3083: f64, t1115: f64, t12201: f64, t14311: f64, t14327: f64, t3917: f64, t4083: f64, t54882: f64, t55962: f64, t57687: f64, t57689: f64, t57694: f64, t57696: f64, t57700: f64, t57702: f64, t57705: f64, t57707: f64, t57711: f64) -> f64 {
    let t58929 = t3083 * t14918;
    let t58940 = t57687 / 12.0_f64 - 7.0_f64 / 24.0_f64 * t57689 - t57694 / 12.0_f64 + 7.0_f64 / 36.0_f64 * t57696 - t12201 * t4083 / 96.0_f64 - t1115 * t54882 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t58929 - t3917 * t14311 / 96.0_f64 - t3917 * t14327 / 96.0_f64 - t57700 / 384.0_f64 + 7.0_f64 / 72.0_f64 * t57702 - t57705 / 12.0_f64 - t55962 - 7.0_f64 / 144.0_f64 * t57707 + t57711 / 384.0_f64;
    t58940
}
