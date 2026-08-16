//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 616/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk616(t2702: f64, t626: f64, t1045: f64, t1055: f64, t184: f64, t188: f64, t2671: f64, t2679: f64, t622: f64, t634: f64, t135: f64, t144: f64, t1501: f64, t1510: f64, t1520: f64, t1530: f64, t1534: f64, t1544: f64, t1547: f64, t1553: f64, t2535: f64, t2536: f64, t2537: f64, t2559: f64, t2575: f64, t2606: f64, t2608: f64, t2611: f64, t560: f64, t637: f64, t639: f64) -> (f64, f64, f64) {
    let t2703 = t626 * t2702;
    let t2706 = 0.65854491829355115987e0_f64 * t2671 * t188 - 0.65854491829355115987e0_f64 * t1045 * t634 - 0.65854491829355115987e0_f64 * t622 * t1055 + 0.13170898365871023197e1_f64 * t184 * t2679 - 0.65854491829355115987e0_f64 * t184 * t2703;
    let t2710 = t135 * t144 * t2706 * t639 + 3.0_f64 * t135 * t2575 * t560 - t2536 * t2537 * t637 - t1501 - t1510 - t1520 + t1530 + t1534 + t1544 - t1547 - t1553 - t2535 + t2559 + t2606 + t2608 - t2611;
    (t2703, t2706, t2710)
}
