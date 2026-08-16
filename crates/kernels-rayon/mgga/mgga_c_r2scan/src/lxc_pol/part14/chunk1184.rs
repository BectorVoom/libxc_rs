//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1184/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1184(t12049: f64, t12053: f64, t12055: f64, t12058: f64, t12061: f64, t12063: f64, t12228: f64, t12081: f64, t12084: f64, t12087: f64, t11198: f64, t11202: f64, t11205: f64, t11324: f64, t11328: f64, t11330: f64) -> f64 {
    let t41126 = t12049 / 2.0_f64;
    let t41127 = t12053 / 2.0_f64;
    let t41128 = 2.0_f64 * t12055;
    let t41129 = 3.0_f64 / 2.0_f64 * t12058;
    let t41130 = 5.0_f64 / 8.0_f64 * t12061;
    let t41131 = 2.0_f64 * t12063;
    let t41132 = 2.0_f64 * t12228;
    let t41133 = 3.0_f64 / 2.0_f64 * t12081;
    let t41134 = t12084 / 2.0_f64;
    let t41135 = t12087 / 2.0_f64;
    let t41136 = -t41126 + t41127 + t41128 + t11198 + t11202 + t41129 + t41130 - t11205 + t11324 + t41131 + t41132 - t11328 + t41133 + t11330 - t41134 + t41135;
    t41136
}
