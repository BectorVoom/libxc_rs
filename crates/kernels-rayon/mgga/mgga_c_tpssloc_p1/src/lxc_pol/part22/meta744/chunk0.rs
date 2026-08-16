//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2468/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2468(t17611: f64, t4641: f64, t10480: f64, t21391: f64, t248: f64, t3101: f64, t1041: f64, t10457: f64, t21118: f64, t1616: f64, t607: f64, t10403: f64, t10408: f64, t10413: f64, t1618: f64, t17151: f64, t17177: f64, t17182: f64, t17923: f64, t3070: f64, t3071: f64, t42397: f64, t42483: f64, t5685: f64, t61744: f64, t61754: f64, t61768: f64, t61782: f64, t62850: f64, t70082: f64, t70086: f64) -> (f64, f64, f64, f64) {
    let t70214 = t4641 * t17611;
    let t70227 = t10480 * t248 * t3101 * t21391;
    let t70239 = t1041 * t248 * t10457 * t21118;
    let t70241 = t1616 * t607;
    let t70268 = 5.0_f64 / 6912.0_f64 * t61744 - t61754 * t1618 / 192.0_f64 - 5.0_f64 / 3456.0_f64 * t70239 - t3070 * t3071 * t17182 * t70241 / 768.0_f64 + t10403 * t3071 * t5685 * t70082 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t17177 * t70241 - t10413 * t3071 * t5685 * t70086 / 1536.0_f64 + t42483 * t3071 * t62850 * t17923 / 1536.0_f64 + 5.0_f64 / 3456.0_f64 * t61768 + 5.0_f64 / 1728.0_f64 * t3070 * t42397 * t17151 * t70241 - t61782 / 6912.0_f64;
    (t70214, t70227, t70241, t70268)
}
