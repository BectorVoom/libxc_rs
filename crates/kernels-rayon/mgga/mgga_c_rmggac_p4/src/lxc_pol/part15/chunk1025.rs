//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1025/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1025(t1550: f64, t46611: f64, t10102: f64, t34884: f64, t1652: f64, t570: f64, t1971: f64, t3351: f64, t875: f64, t1356: f64, t40260: f64, t40263: f64, t46047: f64, t46830: f64, t46834: f64, t46836: f64, t46838: f64, t46841: f64, t46844: f64, t46848: f64, t46853: f64, t46856: f64, t46859: f64, t46861: f64) -> (f64, f64) {
    let t46863 = t1550 * t46611;
    let t46865 = t34884 * t10102;
    let t46867 = t570 * t1652;
    let t46870 = t3351 * t1971 * t875 * t46867;
    let t46872 = 0.17877131955185092547e-3_f64 * t46830 - 0.42564599893297839398e-5_f64 * t46834 + 0.12769379967989351819e-4_f64 * t46836 - 0.12769379967989351819e-4_f64 * t46838 + t40260 - 0.35922725105591425692e0_f64 * t46841 + 0.8980681276397856423e0_f64 * t46844 + 0.17961362552795712846e0_f64 * t46848 + 0.39914139006212695214e-1_f64 * t1356 * t46047 - t40263 - 0.16364796992547205037e0_f64 * t46853 + 0.40911992481368012592e0_f64 * t46856 + 0.81823984962736025184e-1_f64 * t46859 - 0.2993560425465952141e-1_f64 * t46861 - 0.2993560425465952141e-1_f64 * t46863 + 0.74488049813271218945e-4_f64 * t46865 + 0.17025839957319135759e-4_f64 * t46870;
    (t46867, t46872)
}
