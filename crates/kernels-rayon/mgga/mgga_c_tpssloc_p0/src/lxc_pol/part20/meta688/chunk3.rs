//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2608/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2608(t52973: f64, t11801: f64, t5024: f64, t11820: f64, t5019: f64, t11729: f64, t11739: f64, t1227: f64, t15527: f64, t15541: f64, t15545: f64, t15656: f64, t3490: f64, t3536: f64, t44836: f64, t45037: f64, t4582: f64, t45997: f64, t46006: f64, t4977: f64, t4987: f64) -> f64 {
    let t52974 = t52973 / 4608.0_f64;
    let t52975 = t5024 * t11801;
    let t52987 = t5019 * t11820;
    let t52988 = t52987 / 864.0_f64;
    let t52989 = -t44836 * t4582 * t4977 * t11739 / 3072.0_f64 + 5.0_f64 / 2304.0_f64 * t3490 * t15541 + 5.0_f64 / 4608.0_f64 * t3490 * t15545 + 5.0_f64 / 768.0_f64 * t3490 * t15656 + 7.0_f64 / 1536.0_f64 * t45037 * t4582 * t4977 * t11729 - t52974 + t52975 / 216.0_f64 + t3536 * t15527 / 1024.0_f64 + 5.0_f64 / 4608.0_f64 * t1227 * t4582 * t4987 * t46006 + 5.0_f64 / 4608.0_f64 * t1227 * t4582 * t4987 * t45997 + t52988;
    t52989
}
