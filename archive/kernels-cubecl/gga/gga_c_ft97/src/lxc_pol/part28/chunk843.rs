//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 843/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk843<F: Float>(t28: F, t34495: F, t89: F, t32350: F, t920: F, t1564: F, t446: F, t32355: F, t942: F, t34482: F, t370: F, t27: F) -> (F, F, F, F, F, F, F) {
    let t34496 = t28 * t34495;
    let t34497 = t89 * t34496;
    let t34499 = t32350 * t920;
    let t34500 = t1564 * t34499;
    let t34501 = t446 * t34500;
    let t34503 = t32355 * t942;
    let t34504 = t28 * t34503;
    let t34505 = t89 * t34504;
    let t34507 = t370 * t34482;
    let t34509 = t89 * t27 * t34507;
    (t34497, t34500, t34501, t34503, t34505, t34507, t34509)
}
