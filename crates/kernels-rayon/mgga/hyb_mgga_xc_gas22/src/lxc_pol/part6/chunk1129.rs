//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1129/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1129(t4310: f64, t7108: f64, t3605: f64, t2576: f64, t4323: f64, t1007: f64, t1014: f64, t1016: f64, t10979: f64, t11005: f64, t11008: f64, t11016: f64, t11018: f64, t11021: f64, t11024: f64, t11027: f64, t11030: f64, t11034: f64, t11037: f64, t11041: f64, t11075: f64, t11114: f64, t11138: f64, t11168: f64, t11169: f64, t1442: f64, t260: f64, t3591: f64, t3597: f64, t9296: f64) -> (f64, f64, f64, f64, f64) {
    let t11173 = t7108 * t4310;
    let t11174 = t11173 * t3605;
    let t11177 = t2576 * t4323;
    let t11178 = t11177 * t1007;
    let t11181 = -0.5848223622634646207e0_f64 * t10979 * t1016 - 0.5848223622634646207e0_f64 * t1014 * t11005 - 0.35089341735807877242e1_f64 * t1014 * t11008 + 0.23392894490538584828e1_f64 * t3591 * t3597 - 0.11696447245269292414e1_f64 * t9296 * t1442 - t11016 + t11018 + t11021 - t11024 - t11027 - t11030 + t11034 + t11037 + t11041 + t260 * (t11075 + t11114 + t11138 + t11169) + 0.10389515463408878255e3_f64 * t1014 * t11174 + 0.11696447245269292414e1_f64 * t1014 * t11178 - t11168;
    (t11173, t11174, t11177, t11178, t11181)
}
