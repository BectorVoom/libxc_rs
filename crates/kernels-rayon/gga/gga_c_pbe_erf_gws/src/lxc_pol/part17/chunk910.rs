//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 910/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk910(t202: f64, t2814: f64, t184: f64, t619: f64, t1735: f64, t2741: f64, t1672: f64, t996: f64, t561: f64, t2799: f64, t7776: f64, t2768: f64, t418: f64) -> (f64, f64, f64, f64, f64) {
    let t7950 = t202 * t2814;
    let t7951 = t7950 * t184;
    let t7953 = 8.0_f64 / 15.0_f64 * t7951 * t619;
    let t7955 = 4.0_f64 / 15.0_f64 * t2741 * t1735;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    let t7958 = 8.0_f64 / 135.0_f64 * t7957;
    let t7959 = t7776 * t2799;
    let t7960 = t561 * t7959;
    let t7961 = 4.0_f64 / 9.0_f64 * t7960;
    let t7962 = t2768 * t418;
    (t7953, t7955, t7958, t7961, t7962)
}
