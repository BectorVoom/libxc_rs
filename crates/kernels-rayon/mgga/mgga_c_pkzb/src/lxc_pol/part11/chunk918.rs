//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 918/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk918(t3734: f64, t832: f64, t853: f64, t1185: f64, t8214: f64, t3820: f64, t889: f64, t1209: f64, t2252: f64, t2291: f64, t2296: f64, t3793: f64, t3796: f64, t3807: f64, t3823: f64, t6266: f64, t6300: f64, t6308: f64, t8102: f64, t863: f64, t882: f64, t891: f64, t9930: f64, t9959: f64, t9964: f64, t9974: f64) -> (f64, f64, f64, f64, f64) {
    let t9976 = t3734 * t832;
    let t9978 = 1.0_f64 * t9976 * t853;
    let t9980 = 2.0_f64 * t8214 * t1185;
    let t9981 = t3820 * t889;
    let t9984 = -0.11696447245269292414e1_f64 * t6266 * t3807 + 0.5848223622634646207e0_f64 * t2291 * t3820 + 0.5848223622634646207e0_f64 * t882 * t9930 + 0.17315859105681463759e2_f64 * t6300 * t3823 + 1.0_f64 * t2252 * t3793 + 1.0_f64 * t863 * t9959 + 0.32163958997385070134e2_f64 * t6308 * t3796 + 0.5848223622634646207e0_f64 * t9964 * t891 + 0.11696447245269292414e1_f64 * t8102 * t1209 - 0.19751673498613801407e-1_f64 * t9974 - t9978 - t9980 - 0.11696447245269292414e1_f64 * t2296 * t9981;
    (t9976, t9978, t9980, t9981, t9984)
}
