//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 911/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk911<F: Float>(t2417: F, t8697: F, t4911: F, t11030: F, t11033: F, t17382: F, t17385: F, t23472: F, t23481: F, t23570: F, t29088: F, t29094: F, t29116: F, t29121: F, t29124: F, t29126: F, t29139: F) -> (F, F, F) {
    let t29195 = t8697 * t2417;
    let t29196 = t29195 * t4911;
    let t29211 = -F::cast_from(0.16431333333333333333e0_f64) * t29116 - F::cast_from(0.39862222222222222223e0_f64) * t17382 - F::cast_from(0.5477111111111111111e0_f64) * t17385 + F::cast_from(0.98587999999999999998e0_f64) * t29121 + F::cast_from(0.142419375e1_f64) * t29124 - t11030 - t11033 - F::cast_from(0.76790625e-1_f64) * t29126 + F::cast_from(0.1898925e1_f64) * t29139 - F::cast_from(0.65725333333333333332e0_f64) * t23570 - F::cast_from(0.59793333333333333333e0_f64) * t23472 + F::cast_from(0.29896666666666666667e0_f64) * t23481 - F::cast_from(0.59793333333333333333e0_f64) * t29088 + F::cast_from(0.17938e1_f64) * t29094;
    (t29195, t29196, t29211)
}
