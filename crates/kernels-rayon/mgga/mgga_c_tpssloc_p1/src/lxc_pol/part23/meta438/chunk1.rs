//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1282/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1282(t18321: f64, t4916: f64, t11583: f64, t21510: f64, t11570: f64, t15419: f64, t21745: f64, t3447: f64, t20234: f64, t44505: f64, t1171: f64, t22104: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73433 = t18321 * t4916;
    let t73444 = t11583 * t21510;
    let t73451 = t11570 * t21510;
    let t73491 = t3447 * t15419 * t21745;
    let t73496 = t44505 * t20234;
    let t73523 = t22104 * t1171;
    (t73433, t73444, t73451, t73491, t73496, t73523)
}
