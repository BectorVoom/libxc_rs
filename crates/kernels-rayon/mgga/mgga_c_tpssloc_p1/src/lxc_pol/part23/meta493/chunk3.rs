//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1515/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1515(t12419: f64, t19871: f64, t19956: f64, t20448: f64, t20463: f64, t20468: f64, t3803: f64, t3805: f64, t39936: f64, t5246: f64, t74120: f64, t74258: f64, t74260: f64, t74274: f64, t74276: f64, t74297: f64, t74299: f64, t74360: f64, t74376: f64, t74393: f64) -> f64 {
    let t80352 = 7.0_f64 / 96.0_f64 * t74258 + 7.0_f64 / 96.0_f64 * t74260 - t5246 * t3805 * t74120 * t20468 / 32.0_f64 - 7.0_f64 / 48.0_f64 * t74274 + 35.0_f64 / 96.0_f64 * t74276 + t39936 + 7.0_f64 / 1152.0_f64 * t74297 + 7.0_f64 / 1152.0_f64 * t74299 + 7.0_f64 / 3.0_f64 * t74360 + 7.0_f64 / 384.0_f64 * t74376 - 5.0_f64 / 128.0_f64 * t3803 * t12419 * t19956 * t20448 + t3803 * t3805 * t19871 * t20463 / 128.0_f64 - 7.0_f64 / 4.0_f64 * t74393;
    t80352
}
