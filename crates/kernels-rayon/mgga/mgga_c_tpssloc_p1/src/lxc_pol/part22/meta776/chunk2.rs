//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2653/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2653(t12429: f64, t1352: f64, t16394: f64, t1825: f64, t19815: f64, t19871: f64, t19882: f64, t19956: f64, t19972: f64, t19986: f64, t20442: f64, t3803: f64, t3805: f64, t3807: f64, t5245: f64, t5248: f64, t5252: f64, t5287: f64, t56817: f64, t74090: f64, t74110: f64, t74120: f64) -> f64 {
    let t74133 = t3803 * t3805 * t74090 * t3807 / 768.0_f64 - t12429 * t20442 / 1024.0_f64 - t3803 * t5248 * t56817 * t1825 / 1024.0_f64 - t3803 * t5248 * t19956 * t5287 / 1024.0_f64 + t16394 * t19986 / 256.0_f64 + t19815 * t5245 * t5252 / 512.0_f64 - 7.0_f64 / 384.0_f64 * t74110 - t3803 * t5248 * t74090 * t1352 / 3072.0_f64 + t16394 * t19882 / 256.0_f64 - t16394 * t19972 / 512.0_f64 + t3803 * t3805 * t74120 * t3807 / 768.0_f64 - t3803 * t5248 * t19871 * t5287 / 1024.0_f64 - t3803 * t5248 * t74120 * t1352 / 3072.0_f64;
    t74133
}
