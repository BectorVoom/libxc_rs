//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 911/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk911(t2417: f64, t8697: f64, t4911: f64, t11030: f64, t11033: f64, t17382: f64, t17385: f64, t23472: f64, t23481: f64, t23570: f64, t29088: f64, t29094: f64, t29116: f64, t29121: f64, t29124: f64, t29126: f64, t29139: f64) -> (f64, f64, f64) {
    let t29195 = t8697 * t2417;
    let t29196 = t29195 * t4911;
    let t29211 = -0.16431333333333333333e0_f64 * t29116 - 0.39862222222222222223e0_f64 * t17382 - 0.5477111111111111111e0_f64 * t17385 + 0.98587999999999999998e0_f64 * t29121 + 0.142419375e1_f64 * t29124 - t11030 - t11033 - 0.76790625e-1_f64 * t29126 + 0.1898925e1_f64 * t29139 - 0.65725333333333333332e0_f64 * t23570 - 0.59793333333333333333e0_f64 * t23472 + 0.29896666666666666667e0_f64 * t23481 - 0.59793333333333333333e0_f64 * t29088 + 0.17938e1_f64 * t29094;
    (t29195, t29196, t29211)
}
