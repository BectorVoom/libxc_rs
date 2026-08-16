//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 619/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk619(t1020: f64, t192: f64, t1535: f64, t1555: f64, t1596: f64, t1604: f64, t1627: f64, t1629: f64, t1641: f64, t1669: f64, t2613: f64, t2614: f64, t2616: f64, t2618: f64, t2619: f64, t2622: f64, t2711: f64, t2714: f64, t2718: f64, t568: f64) -> (f64, f64) {
    let t2719 = t192 * t1020;
    let t2723 = 3.0_f64 * t1535 * t2714 * t568 + 6.0_f64 * t2718 * t2719 * t568 + 3.0_f64 * t1535 * t2711 - t1555 - t1596 + t1604 + t1627 + t1629 + t1641 + t1669 - t2613 - t2614 + t2616 - t2618 - t2619 - t2622;
    (t2719, t2723)
}
