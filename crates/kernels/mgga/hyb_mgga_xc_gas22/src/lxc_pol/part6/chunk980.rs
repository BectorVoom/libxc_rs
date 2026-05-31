//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 980/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk980<F: Float>(t1005: F, t3580: F, t1434: F, t2593: F, t2578: F, t3583: F, t2601: F, t3579: F, t1433: F, t6996: F, t2560: F, t2577: F, t2599: F, t3532: F, t374: F, t6993: F, t7109: F, t7140: F, t8975: F, t9032: F, t9039: F, t9042: F, t9045: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9048 = t3580 * t1005;
    let t9051 = t1434 * t2593;
    let t9054 = t3583 * t2578;
    let t9057 = t3579 * t2601;
    let t9058 = t9057 * t1005;
    let t9061 = t3583 * t2593;
    let t9064 = t1433 * t6996;
    let t9065 = t9064 * t2578;
    let t9070 = -F::cast_from(0.19751673498613801407e-1_f64) * t9032 - F::cast_from(0.310907e-1_f64) * t9039 * t374 + F::cast_from(0.35089341735807877242e1_f64) * t2599 * t9042 - t8975 + F::cast_from(6.0_f64) * t2560 * t9045 - F::cast_from(0.23392894490538584828e1_f64) * t2577 * t9048 - F::cast_from(0.11696447245269292414e1_f64) * t2577 * t9051 - F::cast_from(0.10389515463408878255e3_f64) * t7109 * t9054 + F::cast_from(0.34631718211362927518e2_f64) * t2599 * t9058 + F::cast_from(0.17315859105681463759e2_f64) * t2599 * t9061 + F::cast_from(0.10254018858216406658e4_f64) * t6993 * t9065 - F::cast_from(4.0_f64) * t7140 * t3532;
    (t9048, t9051, t9054, t9057, t9058, t9061, t9064, t9065, t9070)
}
