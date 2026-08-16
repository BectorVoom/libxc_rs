//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1467/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1467(t11668: f64, t11692: f64, t1735: f64, t1737: f64, t1748: f64, t21762: f64, t21769: f64, t3577: f64, t3578: f64, t467: f64, t5971: f64, t5979: f64, t6219: f64, t6230: f64, t65935: f64, t72304: f64, t72307: f64, t72597: f64, t72600: f64, t72632: f64, t72634: f64, t72648: f64, t78506: f64) -> f64 {
    let t79120 = 5.0_f64 / 2304.0_f64 * t3577 * t11668 * t6219 * t5971 - t72597 / 216.0_f64 - t72600 / 36.0_f64 - t3577 * t3578 * t1735 * t21769 / 192.0_f64 + t11692 * t3578 * t6230 * t5979 / 768.0_f64 - 5.0_f64 / 2304.0_f64 * t11692 * t11668 * t6230 * t5971 + 5.0_f64 / 576.0_f64 * t3577 * t11668 * t1735 * t21762 + 1309.0_f64 / 486.0_f64 * t78506 * t467 - t72632 / 36.0_f64 - t72304 * t1737 / 48.0_f64 - 5.0_f64 / 324.0_f64 * t72634 - 5.0_f64 / 10368.0_f64 * t65935 + t72307 * t1748 / 72.0_f64 - t72648 / 36.0_f64;
    t79120
}
