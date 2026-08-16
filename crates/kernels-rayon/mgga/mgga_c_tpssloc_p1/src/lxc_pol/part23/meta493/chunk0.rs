//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1512/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1512(t225: f64, t80048: f64, t6387: f64, t3792: f64, t40046: f64, t12250: f64, t550: f64, t12419: f64, t16224: f64, t16305: f64, t16394: f64, t1825: f64, t19871: f64, t19876: f64, t19956: f64, t20442: f64, t20460: f64, t20470: f64, t20473: f64, t20563: f64, t28099: f64, t3803: f64, t3805: f64, t5246: f64, t5248: f64, t6330: f64, t6347: f64, t6388: f64, t6394: f64, t6420: f64, t74090: f64, t74120: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80175 = t80048 * t225;
    let t80180 = t6387 * t6387;
    let t80181 = t80180 * t3792;
    let t80185 = t80180 * t40046;
    let t80189 = t80180 * t12250;
    let t80193 = t80180 * t550;
    let t80265 = -t5246 * t16305 * t20473 * t28099 / 32.0_f64 - t3803 * t5248 * t19956 * t6420 / 512.0_f64 - t3803 * t5248 * t74090 * t1825 / 768.0_f64 - t5246 * t3805 * t19871 * t3792 * t6347 / 64.0_f64 - 5.0_f64 / 64.0_f64 * t3803 * t16224 * t20563 * t1825 - t16394 * t20442 / 256.0_f64 + t3803 * t3805 * t74120 * t6394 / 192.0_f64 + t3803 * t3805 * t74090 * t6394 / 192.0_f64 - t19876 * t20470 / 32.0_f64 + 3.0_f64 / 256.0_f64 * t5246 * t5248 * t19956 * t6388 + 5.0_f64 / 64.0_f64 * t5246 * t12419 * t19871 * t3792 * t6330 + t16394 * t20460 / 64.0_f64;
    (t80175, t80181, t80185, t80189, t80193, t80265)
}
