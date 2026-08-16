//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1291/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1291(t8634: f64, t9823: f64, t11119: f64, t23469: f64, t24488: f64, t3470: f64, t24777: f64, t24496: f64, t8478: f64, t9972: f64, t23477: f64, t32613: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33246 = 0.71500979903700853338e0_f64 * t9823 * t8634;
    let t33247 = t23469 * t11119;
    let t33248 = 0.38342925953920749676e0_f64 * t33247;
    let t33253 = 0.10725146985555128001e1_f64 * t24488 * t3470;
    let t33255 = 0.10725146985555128001e1_f64 * t24777 * t3470;
    let t33257 = 0.21450293971110256002e1_f64 * t24496 * t3470;
    let t33259 = 0.21450293971110256002e1_f64 * t8478 * t9972;
    let t33261 = 0.47667319935800568892e0_f64 * t23477 * t32613;
    (t33246, t33248, t33253, t33255, t33257, t33259, t33261)
}
