//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1654/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1654(t1196: f64, t12606: f64, t974: f64, t3548: f64, t4889: f64, t14736: f64, t3440: f64, t14740: f64, t11678: f64, t1174: f64, t11755: f64, t11787: f64, t11792: f64, t11794: f64, t11798: f64, t11802: f64, t11821: f64, t1227: f64, t15650: f64, t15656: f64, t15663: f64) -> f64 {
    let t15666 = t1196 * t12606;
    let t15667 = t974 * t15666;
    let t15671 = t4889 * t3548 / 162.0_f64;
    let t15672 = t3440 * t14736;
    let t15681 = t3440 * t14740;
    let t15684 = -t1227 * t15650 / 1152.0_f64 + t11755 / 648.0_f64 + 5.0_f64 / 2304.0_f64 * t1227 * t15656 - t11678 * t15663 / 1152.0_f64 - t1174 * t15667 / 288.0_f64 + t15671 + t1174 * t15672 / 108.0_f64 + 5.0_f64 / 20736.0_f64 * t11787 + t11792 / 10368.0_f64 + t11794 / 2304.0_f64 - t11798 / 6912.0_f64 - t11802 / 3456.0_f64 - t11821 / 6912.0_f64 + t1174 * t15681 / 216.0_f64;
    t15684
}
