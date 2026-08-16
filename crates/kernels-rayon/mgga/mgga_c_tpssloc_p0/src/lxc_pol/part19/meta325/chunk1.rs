//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1155/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1155(t1314: f64, t9569: f64, t1329: f64, t12189: f64, t3770: f64, t12279: f64, t12303: f64, t12368: f64, t12371: f64, t12419: f64, t1352: f64, t16224: f64, t16401: f64, t3803: f64, t3805: f64, t3806: f64, t3809: f64, t39971: f64, t39973: f64, t39975: f64, t39978: f64, t39983: f64, t39989: f64, t39993: f64, t40000: f64, t5246: f64, t5248: f64) -> (f64, f64) {
    let t40005 = t9569 * t1314;
    let t40006 = t40005 * t1329;
    let t40008 = t12189 * t3770;
    let t40010 = -7.0_f64 / 96.0_f64 * t39971 + 7.0_f64 / 384.0_f64 * t39973 + t39975 * t3809 / 64.0_f64 + 5.0_f64 / 64.0_f64 * t5246 * t12419 * t12368 * t39978 - 7.0_f64 / 192.0_f64 * t39983 - 5.0_f64 / 64.0_f64 * t3803 * t16224 * t1352 * t12303 - 7.0_f64 / 96.0_f64 * t39989 - t16401 * t12371 / 32.0_f64 - t5246 * t3805 * t12368 * t39993 / 64.0_f64 + t16401 * t12279 / 128.0_f64 + t5246 * t5248 * t3806 * t40000 / 384.0_f64 + 455.0_f64 / 162.0_f64 * t40006 - 35.0_f64 / 36.0_f64 * t40008;
    (t40005, t40010)
}
