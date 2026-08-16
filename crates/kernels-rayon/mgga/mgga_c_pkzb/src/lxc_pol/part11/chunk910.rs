//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 910/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk910(t3069: f64, t3073: f64, t2240: f64, t3739: f64, t6201: f64, t851: f64, t6199: f64, t3807: f64, t889: f64, t1209: f64, t3135: f64, t3823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9864 = t3073 * t3069;
    let t9866 = 0.32163958997385070134e2_f64 * t2240 * t9864;
    let t9867 = t3739 * t6201;
    let t9868 = t9867 * t851;
    let t9870 = 0.51726012919273400301e3_f64 * t6199 * t9868;
    let t9875 = t3807 * t889;
    let t9878 = t1209 * t3135;
    let t9881 = t3823 * t889;
    (t9864, t9866, t9867, t9868, t9870, t9875, t9878, t9881)
}
