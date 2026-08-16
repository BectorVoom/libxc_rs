//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 578/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk578(t2080: f64, t2211: f64, t739: f64, t13964: f64, t14092: f64, t14108: f64, t14152: f64, t14269: f64, t14364: f64, t14369: f64, t2339: f64, t3056: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14710 = t2211 * t2080;
    let t14711 = t739 * t14710;
    let t14712 = 0.2993560425465952141e-1_f64 * t14711;
    let t14825 = 0.13010691197123848592e-4_f64 * t13964;
    let t14849 = 0.11400064176674482499e-6_f64 * t14092;
    let t14865 = 0.15965655602485078085e0_f64 * t14108;
    let t14883 = 0.13010691197123848592e-4_f64 * t14152;
    let t14913 = 0.34695176525663596246e-4_f64 * t14269;
    let t14918 = 0.1276937996798935182e-3_f64 * t14364;
    let t14919 = 0.16351352353374609375e-5_f64 * t14369;
    let t15030 = t3056 * t3057 * t2339;
    (t14710, t14712, t14825, t14849, t14865, t14883, t14913, t14918, t14919, t15030)
}
