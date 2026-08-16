//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1096/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1096(t40250: f64, t40262: f64, t43338: f64, t46830: f64, t46834: f64, t46836: f64, t46838: f64, t46841: f64, t46844: f64, t46848: f64, t46853: f64, t46856: f64, t46859: f64, t46861: f64, t46863: f64, t46865: f64, t46870: f64) -> f64 {
    let t48795 = -0.49658699875514145965e-4_f64 * t40250 + 0.35754263910370185096e-3_f64 * t46830 - 0.85129199786595678799e-5_f64 * t46834 + 0.2553875993597870364e-4_f64 * t46836 - 0.2553875993597870364e-4_f64 * t46838 + t43338 - 0.71845450211182851384e0_f64 * t46841 + 0.17961362552795712846e1_f64 * t46844 + 0.35922725105591425692e0_f64 * t46848 - 0.79453919800822633544e-4_f64 * t40262 - 0.32729593985094410076e0_f64 * t46853 + 0.8182398496273602519e0_f64 * t46856 + 0.16364796992547205038e0_f64 * t46859 - 0.5987120850931904282e-1_f64 * t46861 - 0.5987120850931904282e-1_f64 * t46863 + 0.1489760996265424379e-3_f64 * t46865 + 0.3405167991463827152e-4_f64 * t46870;
    t48795
}
