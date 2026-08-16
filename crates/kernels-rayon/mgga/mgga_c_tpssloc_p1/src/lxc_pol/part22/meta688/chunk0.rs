//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2265/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2265(t3263: f64, t5983: f64, t3331: f64, t6031: f64, t18785: f64, t3400: f64, t19262: f64, t3640: f64, t18287: f64, t225: f64, t15419: f64, t18215: f64, t3447: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64257 = t5983 * t3263;
    let t64292 = t6031 * t3331;
    let t64525 = t3400 * t18785;
    let t64548 = t19262 * t3640;
    let t64595 = t18287 * t225;
    let t64624 = t3447 * t15419 * t18215;
    (t64257, t64292, t64525, t64548, t64595, t64624)
}
