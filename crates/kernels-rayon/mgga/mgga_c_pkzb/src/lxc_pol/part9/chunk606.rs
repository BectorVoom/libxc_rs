//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 606/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk606(t1556: f64, t1631: f64, t1009: f64, t496: f64, t501: f64, t1671: f64, t1008: f64, t46: f64, t552: f64, t1555: f64, t1596: f64, t1604: f64, t1627: f64, t1629: f64, t1641: f64, t1669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2613 = 4.0_f64 * t1556;
    let t2614 = 4.0_f64 * t1631;
    let t2615 = t496 * t1009;
    let t2616 = 4.0_f64 * t2615;
    let t2617 = t501 * t1009;
    let t2618 = 4.0_f64 * t2617;
    let t2619 = 0.18311447306006545054e-3_f64 * t1671;
    let t2620 = t1008 * t46;
    let t2621 = t2620 * t552;
    let t2622 = 0.18311447306006545054e-3_f64 * t2621;
    let t2623 = -t1555 - t2613 + t1627 + t1629 - t2614 + t2616 - t2618 + t1604 + t1641 - t1596 + t1669 - t2619 - t2622;
    (t2613, t2614, t2616, t2618, t2619, t2620, t2622, t2623)
}
