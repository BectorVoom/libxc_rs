//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1071/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1071<F: Float>(t4310: F, t7108: F, t3605: F, t2576: F, t4323: F, t1007: F, t1014: F, t1016: F, t10979: F, t11005: F, t11008: F, t11016: F, t11018: F, t11021: F, t11024: F, t11027: F, t11030: F, t11034: F, t11037: F, t11041: F, t11075: F, t11114: F, t11138: F, t11168: F, t11169: F, t1442: F, t260: F, t3591: F, t3597: F, t9296: F) -> (F, F, F, F, F) {
    let t11173 = t7108 * t4310;
    let t11174 = t11173 * t3605;
    let t11177 = t2576 * t4323;
    let t11178 = t11177 * t1007;
    let t11181 = -0.5848223622634646207e0 * t10979 * t1016 - 0.5848223622634646207e0 * t1014 * t11005 - 0.35089341735807877242e1 * t1014 * t11008 + 0.23392894490538584828e1 * t3591 * t3597 - 0.11696447245269292414e1 * t9296 * t1442 - t11016 + t11018 + t11021 - t11024 - t11027 - t11030 + t11034 + t11037 + t11041 + t260 * (t11075 + t11114 + t11138 + t11169) + 0.10389515463408878255e3 * t1014 * t11174 + 0.11696447245269292414e1 * t1014 * t11178 - t11168;
    (t11173, t11174, t11177, t11178, t11181)
}
