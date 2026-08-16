//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1456/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1456(t11714: f64, t476: f64, t42341: f64, t44696: f64, t3508: f64, t23508: f64, t11883: f64, t3493: f64, t11889: f64, t11620: f64, t11638: f64, t11639: f64, t11877: f64, t11881: f64, t11888: f64, t11893: f64, t11904: f64, t11914: f64, t11915: f64, t1235: f64, t1244: f64, t1246: f64, t1247: f64, t3610: f64, t3611: f64, t3617: f64, t3624: f64, t3625: f64, t44673: f64, t44700: f64, t44707: f64, t44710: f64, t5068: f64) -> (f64, f64, f64) {
    let t44722 = 1.0_f64 / t11714 / t476;
    let t44724 = t44696 * t42341 * t44722;
    let t44725 = t3508 * t3508;
    let t44726 = t23508 * t44725;
    let t44730 = t11883 * t3493;
    let t44741 = t11889 * t3493;
    let t44748 = 4.0_f64 * t11638 * t1235 * t1244 * t1246 + 24.0_f64 * t11620 * t3610 * t5068 + 8.0_f64 * t11639 * t3610 * t5068 + 36.0_f64 * t11881 * t3611 * t44730 - 36.0_f64 * t11888 * t3611 * t44741 + 4.0_f64 * t11914 * t11915 * t44673 - 6.0_f64 * t3624 * t3625 * t44710 + 24.0_f64 * t44700 * t44724 * t44726 + 12.0_f64 * t11877 * t3617 + 24.0_f64 * t11893 * t11904 + 4.0_f64 * t1247 * t44707;
    (t44722, t44725, t44748)
}
