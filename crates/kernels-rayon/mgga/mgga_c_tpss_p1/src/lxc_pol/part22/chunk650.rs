//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 650/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk650(t2975: f64, t3001: f64, t1054: f64, t1063: f64, t1073: f64, t1082: f64, t2856: f64, t2859: f64, t2866: f64, t2908: f64, t2916: f64, t2922: f64, t2925: f64, t2930: f64, t2932: f64, t2950: f64, t2955: f64, t2958: f64, t2967: f64, t2969: f64, t2974: f64, t2976: f64, t2994: f64, t2999: f64, t421: f64) -> (f64, f64) {
    let t3002 = t2975 * t3001;
    let t3005 = -0.310907e-1_f64 * t2922 * t421 + 2.0_f64 * t2925 * t1063 - 2.0_f64 * t2930 * t2932 + 1.0_f64 * t1054 * t2950 + 0.32163958997385070134e2_f64 * t2955 * t2958 + t2856 - t2859 + t2866 - t2908 - t2916 - 0.19751673498613801407e-1_f64 * t2967 + 0.11696447245269292414e1_f64 * t2969 * t1082 - 0.11696447245269292414e1_f64 * t2974 * t2976 + 0.5848223622634646207e0_f64 * t1073 * t2994 + 0.17315859105681463759e2_f64 * t2999 * t3002;
    (t3002, t3005)
}
