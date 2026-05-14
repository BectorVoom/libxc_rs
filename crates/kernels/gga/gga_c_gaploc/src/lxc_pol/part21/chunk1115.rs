//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1115/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1115<F: Float>(t5679: F, t7682: F, t8792: F, t2628: F, t8521: F, t8634: F, t9823: F, t11119: F, t23469: F, t24488: F, t3470: F, t24777: F, t24496: F, t8478: F, t9972: F, t23477: F, t32613: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33238 = 0.21450293971110256002e1 * t5679 * t8792 * t7682;
    let t33239 = t8521 * t2628;
    let t33240 = 0.59584149919750711116e-1 * t33239;
    let t33246 = 0.71500979903700853338e0 * t9823 * t8634;
    let t33247 = t23469 * t11119;
    let t33248 = 0.38342925953920749676e0 * t33247;
    let t33253 = 0.10725146985555128001e1 * t24488 * t3470;
    let t33255 = 0.10725146985555128001e1 * t24777 * t3470;
    let t33257 = 0.21450293971110256002e1 * t24496 * t3470;
    let t33259 = 0.21450293971110256002e1 * t8478 * t9972;
    let t33261 = 0.47667319935800568892e0 * t23477 * t32613;
    (t33238, t33240, t33246, t33248, t33253, t33255, t33257, t33259, t33261)
}
