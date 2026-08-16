//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1066/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1066(t1585: f64, t1588: f64, t1515: f64, t1528: f64, t479: f64, t490: f64, t16200: f64, t16202: f64, t16205: f64, t16208: f64, t16210: f64, t16215: f64, t16217: f64, t16219: f64, t16221: f64, t16224: f64, t472: f64, t491: f64) -> (f64, f64, f64, f64) {
    let t16673 = t1585 * t1585;
    let t16676 = t1588 * t1588;
    let t16701 = 0.4274e0_f64 * t479 * t1515 * t490 * t1528;
    let t16721 = 1.0_f64 * t472 * (-0.21099166666666666667e1_f64 * t16200 + 0.202552e2_f64 * t16202 - 0.75019259259259259258e1_f64 * t16205 + 0.6564185185185185185e1_f64 * t16208 + 0.31003950617283950618e1_f64 * t16210 + 0.68258333333333333335e-1_f64 * t16215 - 0.10921333333333333333e1_f64 * t16217 + 0.12134814814814814815e1_f64 * t16219 + 0.10617962962962962963e1_f64 * t16221 + 0.13388493827160493828e1_f64 * t16224) * t491;
    (t16673, t16676, t16701, t16721)
}
