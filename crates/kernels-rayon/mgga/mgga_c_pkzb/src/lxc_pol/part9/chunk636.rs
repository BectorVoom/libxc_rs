//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 636/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk636(t2852: f64, t721: f64, t1096: f64, t1108: f64, t1911: f64, t1916: f64, t1938: f64, t1950: f64, t1955: f64, t1977: f64, t248: f64, t2745: f64, t2748: f64, t2750: f64, t2753: f64, t2785: f64, t2789: f64, t2793: f64, t2796: f64, t2801: f64, t2816: f64, t2820: f64, t2827: f64, t2829: f64, t2834: f64, t2849: f64, t695: f64, t704: f64, t714: f64, t723: f64) -> (f64, f64) {
    let t2853 = t2852 * t721;
    let t2856 = -0.310907e-1_f64 * t2793 * t248 + 1.0_f64 * t2796 * t704 + 1.0_f64 * t1911 * t1096 - 2.0_f64 * t1916 * t2801 + 1.0_f64 * t695 * t2816 + 0.32163958997385070134e2_f64 * t1938 * t2820 + t2745 - t2748 - t2750 + t2753 - t2785 - t2789 - 0.19751673498613801407e-1_f64 * t2827 + 0.5848223622634646207e0_f64 * t2829 * t723 + 0.5848223622634646207e0_f64 * t1950 * t1108 - 0.11696447245269292414e1_f64 * t1955 * t2834 + 0.5848223622634646207e0_f64 * t714 * t2849 + 0.17315859105681463759e2_f64 * t1977 * t2853;
    (t2853, t2856)
}
