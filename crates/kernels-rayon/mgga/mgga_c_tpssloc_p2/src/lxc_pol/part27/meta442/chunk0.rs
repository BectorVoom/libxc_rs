//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1773/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1773(t22916: f64, t6889: f64, t6888: f64, t22674: f64, t6891: f64, t22892: f64, t1988: f64, t22716: f64, t22724: f64, t6898: f64, t6902: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22917 = t6889 * t22916;
    let t22918 = t6888 * t22917;
    let t22920 = t22674 * t6891;
    let t22921 = t22892 * t22920;
    let t22922 = 0.16449340668482264365e-1_f64 * t22921;
    let t22923 = t22716 * t1988;
    let t22924 = 0.63969658155208805863e-1_f64 * t22923;
    let t22925 = t22724 * t6898;
    let t22926 = 0.26044789391763585244e-1_f64 * t22925;
    let t22927 = t794 * t6902;
    (t22917, t22918, t22920, t22921, t22922, t22924, t22926, t22927)
}
