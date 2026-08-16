//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1302/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1302(t3179: f64, t51291: f64, t854: f64, t51244: f64, t54075: f64, t54077: f64, t54080: f64, t54082: f64, t54085: f64, t54088: f64, t54092: f64, t54094: f64, t54096: f64, t54098: f64) -> f64 {
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54103 = 7.0_f64 / 72.0_f64 * t54102;
    let t54104 = -t54075 / 48.0_f64 + t54077 / 768.0_f64 - t54080 / 48.0_f64 + t54082 / 48.0_f64 - t54085 / 48.0_f64 + t54088 - t54092 / 12.0_f64 + 35.0_f64 / 432.0_f64 * t54094 - t54096 / 768.0_f64 + t54098 / 128.0_f64 - 7.0_f64 / 288.0_f64 * t51244 + t54103;
    t54104
}
