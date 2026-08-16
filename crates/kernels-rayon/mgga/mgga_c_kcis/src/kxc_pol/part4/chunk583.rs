//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 583/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk583(t278: f64, t3038: f64, t1001: f64, t286: f64, t1005: f64, t285: f64, t2867: f64, t2870: f64, t2872: f64, t2879: f64, t2882: f64, t2885: f64, t2891: f64, t2896: f64, t2901: f64, t2905: f64, t2913: f64, t293: f64, t984: f64, t991: f64, t996: f64) -> (f64, f64, f64) {
    let t288 = 0.0_f64 < t278;
    let t3040 = piecewise3(t288, t3038, -t3038);
    let t3041 = t1001 * t3040;
    let t3042 = t286 * t3041;
    let t3045 = 11.0_f64 / 108.0_f64 * t2867 * t293 - t2870 / 54.0_f64 - t2872 * t996 / 54.0_f64 + t984 * t1005 / 18.0_f64 - t2879 + t2882 / 432.0_f64 - t2885 / 144.0_f64 + t991 * t2891 / 216.0_f64 - t991 * t2896 / 144.0_f64 - t991 * t2901 / 144.0_f64 + t991 * t2905 / 288.0_f64 + t285 * t2913 / 48.0_f64 - t285 * t3042 / 96.0_f64;
    (t3040, t3041, t3045)
}
