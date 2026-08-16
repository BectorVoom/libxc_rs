//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1183/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1183(t40389: f64, t40437: f64, t225: f64, t3774: f64, t3862: f64, t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t12368: f64, t12369: f64, t12402: f64, t12407: f64, t12419: f64, t12420: f64, t12422: f64, t12426: f64, t12429: f64, t1352: f64, t16233: f64, t16305: f64, t3803: f64, t3805: f64, t3807: f64, t40183: f64, t40197: f64, t40304: f64, t40329: f64, t40335: f64, t5246: f64, t5248: f64, t5250: f64, t554: f64, t559: f64) -> (f64, f64, f64, f64) {
    let t40438 = t40389 + t40437;
    let t40439 = t40438 * t225;
    let t40443 = t3774 * t3862;
    let t40445 = t6597 * t241;
    let t40449 = 13685.0_f64 / 31104.0_f64 * t555 * t40445 * t557 * t248;
    let t40450 = t3803 * t3805 * t40304 * t3807 / 192.0_f64 - t3803 * t5248 * t40304 * t1352 / 768.0_f64 - 5.0_f64 / 64.0_f64 * t12429 * t12422 - 5.0_f64 / 128.0_f64 * t3803 * t12419 * t12402 * t12420 + t12429 * t12426 / 64.0_f64 + t3803 * t3805 * t12368 * t12407 / 128.0_f64 - t5246 * t16305 * t5250 * t40197 / 32.0_f64 - 7.0_f64 / 1152.0_f64 * t40329 - t5246 * t3805 * t40183 * t12369 / 32.0_f64 - 3.0_f64 / 256.0_f64 * t16233 * t5248 * t12368 * t40335 + t40439 * t554 * t559 / 3072.0_f64 + 119.0_f64 / 2304.0_f64 * t40443 + t40449;
    (t40438, t40439, t40445, t40450)
}
