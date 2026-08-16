//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2575/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2575(t14749: f64, t15402: f64, t3447: f64, t11499: f64, t11505: f64, t44424: f64, t44439: f64, t44504: f64, t4889: f64, t52050: f64, t52053: f64, t52058: f64, t52061: f64, t52064: f64, t52066: f64, t52074: f64, t52076: f64, t52081: f64, t52085: f64, t52086: f64, t52089: f64) -> f64 {
    let t52092 = t3447 * t15402 * t14749;
    let t52094 = 0.37037037037037037036e-3_f64 * t52050 + 0.55555555555555555554e-3_f64 * t52053 + t52058 - 0.37037037037037037036e-3_f64 * t52061 + 0.74074074074074074072e-3_f64 * t52064 + 0.25925925925925925925e-2_f64 * t3447 * t44504 * t52066 + 0.55555555555555555554e-3_f64 * t44424 + 0.55555555555555555554e-3_f64 * t44439 + 0.22222222222222222222e-2_f64 * t4889 * t11505 - 0.14814814814814814815e-2_f64 * t52074 + 0.22222222222222222222e-2_f64 * t52076 + 0.22222222222222222222e-2_f64 * t4889 * t11499 - 0.3086419753086419753e-3_f64 * t52081 + t52085 + 0.22222222222222222222e-2_f64 * t52086 + 0.55555555555555555554e-3_f64 * t52089 - 0.11111111111111111111e-2_f64 * t52092;
    t52094
}
