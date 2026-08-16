//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 876/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk876(t2227: f64, t558: f64, t1587: f64, t698: f64, t41523: f64, t41531: f64, t41534: f64, t41536: f64, t41549: f64, t42144: f64, t42151: f64, t42166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44232 = t2227 * t558;
    let t44239 = t698 * t1587;
    let t44337 = 0.47896966807455234256e0_f64 * t41523;
    let t44339 = 0.95793933614910468512e0_f64 * t41531;
    let t44340 = 0.19158786722982093702e1_f64 * t41534;
    let t44341 = 0.47896966807455234256e0_f64 * t41536;
    let t44362 = 0.3193131120497015617e0_f64 * t41549;
    let t44382 = 0.49658699875514145965e-4_f64 * t42144;
    let t44385 = 0.47896966807455234256e0_f64 * t42151;
    let t44396 = 0.21819729323396273384e0_f64 * t42166;
    (t44232, t44239, t44337, t44339, t44340, t44341, t44362, t44382, t44385, t44396)
}
