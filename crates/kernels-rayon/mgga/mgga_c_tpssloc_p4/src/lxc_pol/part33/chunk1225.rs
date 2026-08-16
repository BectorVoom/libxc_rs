//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1225/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1225(t22644: f64, t81152: f64, t1988: f64, t81071: f64, t225: f64, t22643: f64, t1987: f64, t81144: f64, t9537: f64, t107: f64, t835: f64, t240: f64, t656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81281 = t81152 * t22644;
    let t81282 = 0.98696044010893586188e-1_f64 * t81281;
    let t81317 = t81071 * t1988;
    let t81318 = 0.27720185200590482541e0_f64 * t81317;
    let t81326 = t22643 * t225;
    let t81398 = t81144 * t9537 * t1987;
    let t81399 = 0.13707783890401886971e-2_f64 * t81398;
    let t81437 = t835 * t107;
    let t81438 = 154.0_f64 / 27.0_f64 * t81437;
    let t81439 = t240 * t656;
    (t81282, t81318, t81326, t81399, t81438, t81439)
}
