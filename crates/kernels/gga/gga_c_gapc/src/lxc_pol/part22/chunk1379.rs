//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1379/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1379<F: Float>(t33743: F, t33746: F, t33750: F, t33753: F, t33755: F, t33758: F, t33760: F, t33763: F, t33770: F, t33772: F, t33774: F, t33777: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36688 = F::cast_from(0.1351988360087076823e-6_f64) * t33743;
    let t36689 = F::cast_from(0.21102562238076876322e-7_f64) * t33746;
    let t36690 = F::cast_from(0.40021712703254065176e-7_f64) * t33750;
    let t36691 = F::cast_from(0.80043425406508130352e-7_f64) * t33753;
    let t36692 = F::cast_from(0.32826207925897363168e-8_f64) * t33755;
    let t36693 = F::cast_from(0.49520679385353736436e-5_f64) * t33758;
    let t36694 = F::cast_from(0.13259130899812740005e-6_f64) * t33760;
    let t36695 = F::cast_from(0.44197102999375800018e-8_f64) * t33763;
    let t36698 = F::cast_from(0.10567613244746075633e-6_f64) * t33770;
    let t36699 = F::cast_from(0.40021712703254065176e-7_f64) * t33772;
    let t36700 = F::cast_from(0.40094868252346065012e-6_f64) * t33774;
    let t36701 = F::cast_from(0.66295654499063700026e-7_f64) * t33777;
    (t36688, t36689, t36690, t36691, t36692, t36693, t36694, t36695, t36698, t36699, t36700, t36701)
}
