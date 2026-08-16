//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 822/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk822(t2139: f64, t7360: f64, t2122: f64, t2187: f64, t5098: f64, t5101: f64, t5106: f64, t5108: f64, t6106: f64, t6132: f64, t6139: f64, t6293: f64, t6583: f64, t7312: f64, t7313: f64, t7317: f64, t7323: f64, t7327: f64, t7330: f64, t7334: f64, t7341: f64, t7346: f64, t7349: f64, t7353: f64, t7357: f64) -> f64 {
    let t7362 = 0.69345773920434148506e0_f64 * t2139 * t7360;
    let t7363 = t7312 + 0.86682217400542685632e-1_f64 * t7313 * t2187 + t7317 + 0.69861909304693186868e-1_f64 * t5098 - 0.32927245914677557994e-1_f64 * t5101 + 0.11643651550782197811e-1_f64 * t5106 - 0.32927245914677557994e0_f64 * t6293 * t7323 - 0.17336443480108537126e0_f64 * t6583 * t7327 - 0.10401866088065122276e1_f64 * t6106 * t7330 - 0.2600466522016280569e0_f64 * t5108 * t7334 - 0.21951497276451705328e0_f64 * t2122 * t7341 - 0.17336443480108537126e0_f64 * t6132 * t7346 - 0.5200933044032561138e0_f64 * t6139 * t7349 - 0.2600466522016280569e0_f64 * t5108 * t7353 + 0.10975748638225852664e0_f64 * t2122 * t7357 - t7362;
    t7363
}
