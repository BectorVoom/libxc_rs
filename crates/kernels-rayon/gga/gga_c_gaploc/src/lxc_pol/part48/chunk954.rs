//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 954/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk954(t41666: f64, t41669: f64, t41674: f64, t2478: f64, t3541: f64, t6583: f64, t10612: f64, t13445: f64, t13446: f64, t13450: f64, t13453: f64, t13458: f64, t1580: f64, t1599: f64, t1641: f64, t193: f64, t41672: f64, t44405: f64, t44601: f64, t44609: f64, t46080: f64, t46118: f64, t46119: f64, t46125: f64, t46127: f64, t4820: f64, t524: f64, t531: f64, t541: f64, t557: f64, t568: f64, t569: f64, t574: f64, t597: f64, t600: f64, t6717: f64, t6824: f64, t6914: f64, t8248: f64) -> f64 {
    let t46128 = 0.41708904943825497782e0_f64 * t41666;
    let t46129 = 0.11916829983950142223e0_f64 * t41669;
    let t46131 = 0.15337170381568299871e1_f64 * t41674;
    let t46138 = t6583 * t3541 * t2478;
    let t46162 = -0.12423108009070322895e3_f64 * t6914 * t6717 * t46080 - t46118 + 0.89376224879626066676e-1_f64 * t46119 - t46125 + t46127 - t46128 - t46129 - 0.15337170381568299871e1_f64 * t41672 - t46131 + 0.23833659967900284447e0_f64 * t8248 * t10612 - 0.15889106645266856298e0_f64 * t6824 * t4820 * t44405 - 0.95857314884801874192e0_f64 * t46138 - 0.35750489951850426669e0_f64 * t1599 * t13453 - 0.35750489951850426669e0_f64 * t557 * t531 * t44609 + 0.23833659967900284446e0_f64 * t13446 * t541 + 0.23005755572352449806e1_f64 * t1580 * t13450 + 0.23005755572352449806e1_f64 * t597 * t568 * t600 * t44601 - 0.23005755572352449806e1_f64 * t1641 * t13458 - 0.23005755572352449806e1_f64 * t574 * t568 * t569 * t44601 + 0.35750489951850426669e0_f64 * t524 * t13445 * t193;
    t46162
}
