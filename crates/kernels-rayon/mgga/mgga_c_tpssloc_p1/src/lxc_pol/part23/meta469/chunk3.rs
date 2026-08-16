//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1389/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1389(t1409: f64, t20217: f64, t10403: f64, t10408: f64, t1041: f64, t13995: f64, t14164: f64, t14187: f64, t1539: f64, t1616: f64, t21396: f64, t21403: f64, t21512: f64, t21520: f64, t21526: f64, t21551: f64, t3039: f64, t3070: f64, t3071: f64, t42483: f64, t43361: f64, t4582: f64, t4588: f64, t4644: f64, t49929: f64, t5677: f64, t5681: f64, t5867: f64, t5873: f64, t62284: f64, t70391: f64, t70535: f64, t70554: f64, t70573: f64, t70597: f64, t77606: f64) -> (f64, f64) {
    let t77621 = t20217 * t1409;
    let t77637 = 5.0_f64 / 1152.0_f64 * t10403 * t10408 * t5873 * t5677 - t43361 * t3071 * t21396 * t1539 / 192.0_f64 + t49929 * t21526 / 192.0_f64 - t13995 * t21520 / 192.0_f64 - t3070 * t3071 * t5681 * t5867 / 384.0_f64 + t1041 * t4582 * t14164 * t77606 / 128.0_f64 + t70535 / 288.0_f64 + t70554 / 384.0_f64 - t3039 * t4582 * t70391 * t1616 / 768.0_f64 + 5.0_f64 / 1728.0_f64 * t70573 - t62284 / 1728.0_f64 + 5.0_f64 / 1152.0_f64 * t4644 * t21512 + 5.0_f64 / 3456.0_f64 * t1041 * t4582 * t4588 * t77621 + 5.0_f64 / 864.0_f64 * t1041 * t4582 * t14187 * t77606 - t4644 * t21551 / 192.0_f64 - t70597 / 384.0_f64 + t42483 * t3071 * t21403 * t1539 / 1152.0_f64;
    (t77621, t77637)
}
