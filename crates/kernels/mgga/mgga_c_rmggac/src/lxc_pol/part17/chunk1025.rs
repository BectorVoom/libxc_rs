//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1025/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1025<F: Float>(t1550: F, t46611: F, t10102: F, t34884: F, t1652: F, t570: F, t1971: F, t3351: F, t875: F, t1356: F, t40260: F, t40263: F, t46047: F, t46830: F, t46834: F, t46836: F, t46838: F, t46841: F, t46844: F, t46848: F, t46853: F, t46856: F, t46859: F, t46861: F) -> (F, F) {
    let t46863 = t1550 * t46611;
    let t46865 = t34884 * t10102;
    let t46867 = t570 * t1652;
    let t46870 = t3351 * t1971 * t875 * t46867;
    let t46872 = F::new(0.17877131955185092547e-3) * t46830 - F::new(0.42564599893297839398e-5) * t46834 + F::new(0.12769379967989351819e-4) * t46836 - F::new(0.12769379967989351819e-4) * t46838 + t40260 - F::new(0.35922725105591425692e0) * t46841 + F::new(0.8980681276397856423e0) * t46844 + F::new(0.17961362552795712846e0) * t46848 + F::new(0.39914139006212695214e-1) * t1356 * t46047 - t40263 - F::new(0.16364796992547205037e0) * t46853 + F::new(0.40911992481368012592e0) * t46856 + F::new(0.81823984962736025184e-1) * t46859 - F::new(0.2993560425465952141e-1) * t46861 - F::new(0.2993560425465952141e-1) * t46863 + F::new(0.74488049813271218945e-4) * t46865 + F::new(0.17025839957319135759e-4) * t46870;
    (t46867, t46872)
}
