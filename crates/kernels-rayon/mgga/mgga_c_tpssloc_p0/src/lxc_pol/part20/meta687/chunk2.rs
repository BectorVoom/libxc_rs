//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2604/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2604(t13969: f64, t15621: f64, t3506: f64, t11791: f64, t5005: f64, t11697: f64, t15477: f64, t3577: f64, t11677: f64, t15027: f64, t11680: f64, t11684: f64, t1174: f64, t11751: f64, t1177: f64, t1227: f64, t15740: f64, t3440: f64, t4582: f64, t45997: f64, t4889: f64, t4972: f64, t50873: f64, t50884: f64, t50959: f64, t50964: f64) -> f64 {
    let t52859 = t3506 * t13969 * t15621;
    let t52872 = t5005 * t11791;
    let t52873 = t52872 / 6912.0_f64;
    let t52875 = t3577 * t11697 * t15477;
    let t52879 = t15027 * t11677;
    let t52886 = -t1174 * t1177 * t50873 / 144.0_f64 + t52859 / 768.0_f64 + t4889 * t11751 / 18.0_f64 + t1174 * t3440 * t50884 / 72.0_f64 + t1174 * t3440 * t50959 / 72.0_f64 + t1174 * t3440 * t50964 / 12.0_f64 + t52873 - t52875 / 1152.0_f64 - t15740 * t11684 / 1536.0_f64 - t52879 * t11680 / 768.0_f64 - t1227 * t4582 * t4972 * t45997 / 768.0_f64;
    t52886
}
