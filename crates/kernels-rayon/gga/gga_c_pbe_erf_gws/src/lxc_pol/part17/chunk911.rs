//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 911/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk911(t7694: f64, t7962: f64, t1820: f64, t2575: f64, t4934: f64, t1620: f64, t2826: f64, t583: f64, t5564: f64, t5562: f64, t7927: f64, t7931: f64, t7934: f64, t7939: f64, t7943: f64, t7944: f64, t7947: f64, t7949: f64, t7953: f64, t7955: f64, t7958: f64, t7961: f64) -> (f64, f64, f64, f64, f64) {
    let t7963 = t7694 * t7962;
    let t7965 = 16.0_f64 / 45.0_f64 * t1820 * t7963;
    let t7966 = t4934 * t2575;
    let t7968 = 32.0_f64 / 135.0_f64 * t1620 * t7966;
    let t7970 = 8.0_f64 / 45.0_f64 * t2826 * t583;
    let t7971 = 8.0_f64 / 45.0_f64 * t5564;
    let t7972 = t7927 - t7931 + t7934 - t7939 - t7943 - t7944 - t7947 + t5562 - t7949 + t7953 + t7955 - t7958 - t7961 + t7965 + t7968 + t7970 + t7971;
    (t7965, t7968, t7970, t7971, t7972)
}
