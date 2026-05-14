//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 757/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk757<F: Float>(t34482: F, t469: F, t1317: F, t28: F, t32333: F, t7824: F, t920: F, t446: F, t32338: F, t942: F, t89: F, t5507: F, t6454: F, t32350: F, t1564: F, t32355: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34483 = t469 * t34482;
    let t34485 = t1317 * t28 * t34483;
    let t34488 = t7824 * t32333 * t920;
    let t34489 = t446 * t34488;
    let t34491 = t32338 * t942;
    let t34492 = t28 * t34491;
    let t34493 = t89 * t34492;
    let t34495 = t5507 * t6454;
    let t34496 = t28 * t34495;
    let t34497 = t89 * t34496;
    let t34499 = t32350 * t920;
    let t34500 = t1564 * t34499;
    let t34501 = t446 * t34500;
    let t34503 = t32355 * t942;
    (t34483, t34485, t34488, t34489, t34491, t34493, t34495, t34497, t34500, t34501, t34503)
}
