//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 980/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk980(t1005: f64, t3580: f64, t1434: f64, t2593: f64, t2578: f64, t3583: f64, t2601: f64, t3579: f64, t1433: f64, t6996: f64, t2560: f64, t2577: f64, t2599: f64, t3532: f64, t374: f64, t6993: f64, t7109: f64, t7140: f64, t8975: f64, t9032: f64, t9039: f64, t9042: f64, t9045: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9048 = t3580 * t1005;
    let t9051 = t1434 * t2593;
    let t9054 = t3583 * t2578;
    let t9057 = t3579 * t2601;
    let t9058 = t9057 * t1005;
    let t9061 = t3583 * t2593;
    let t9064 = t1433 * t6996;
    let t9065 = t9064 * t2578;
    let t9070 = -0.19751673498613801407e-1_f64 * t9032 - 0.310907e-1_f64 * t9039 * t374 + 0.35089341735807877242e1_f64 * t2599 * t9042 - t8975 + 6.0_f64 * t2560 * t9045 - 0.23392894490538584828e1_f64 * t2577 * t9048 - 0.11696447245269292414e1_f64 * t2577 * t9051 - 0.10389515463408878255e3_f64 * t7109 * t9054 + 0.34631718211362927518e2_f64 * t2599 * t9058 + 0.17315859105681463759e2_f64 * t2599 * t9061 + 0.10254018858216406658e4_f64 * t6993 * t9065 - 4.0_f64 * t7140 * t3532;
    (t9048, t9051, t9054, t9057, t9058, t9061, t9064, t9065, t9070)
}
