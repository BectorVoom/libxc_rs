//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 320/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk320(t1175: f64, t359: f64, t375: f64, t1130: f64, t1133: f64, t376: f64, t1085: f64, t355: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1176 = t1175 * t359;
    let t1177 = t375 * t1176;
    let t1179 = t1130 * t1133;
    let t1180 = t376 * t1179;
    let t1181 = t375 * t1180;
    let t1183 = t1085 * t355;
    let t1184 = t1183 * t381;
    (t1176, t1177, t1179, t1180, t1181, t1183, t1184)
}
