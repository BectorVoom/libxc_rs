//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1132/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1132(t18583: f64, t3578: f64, t17691: f64, t4972: f64, t4582: f64, t15615: f64, t17686: f64, t1155: f64, t6069: f64, t1695: f64, t4857: f64, t6088: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18584 = t3578 * t18583;
    let t18589 = t4972 * t17691;
    let t18590 = t4582 * t18589;
    let t18593 = t15615 * t17686;
    let t18594 = t4582 * t18593;
    let t18603 = t6069 * t1155;
    let t18606 = t1695 * t4857;
    let t18609 = t6088 * t1155;
    (t18584, t18590, t18594, t18603, t18606, t18609)
}
