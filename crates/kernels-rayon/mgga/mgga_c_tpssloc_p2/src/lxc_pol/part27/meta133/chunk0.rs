//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 763/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk763(t2906: f64, t2932: f64, t2786: f64, t2789: f64, t2796: f64, t2839: f64, t2847: f64, t2853: f64, t2856: f64, t2861: f64, t2863: f64, t2881: f64, t2886: f64, t2889: f64, t2898: f64, t2900: f64, t2905: f64, t2907: f64, t2925: f64, t2930: f64, t311: f64, t924: f64, t933: f64, t943: f64, t952: f64) -> (f64, f64) {
    let t2933 = t2906 * t2932;
    let t2936 = -0.310907e-1_f64 * t2853 * t311 + 2.0_f64 * t2856 * t933 - 2.0_f64 * t2861 * t2863 + 1.0_f64 * t924 * t2881 + 0.32163958997385070134e2_f64 * t2886 * t2889 + t2786 - t2789 + t2796 - t2839 - t2847 - 0.19751673498613801407e-1_f64 * t2898 + 0.11696447245269292414e1_f64 * t2900 * t952 - 0.11696447245269292414e1_f64 * t2905 * t2907 + 0.5848223622634646207e0_f64 * t943 * t2925 + 0.17315859105681463759e2_f64 * t2930 * t2933;
    (t2933, t2936)
}
