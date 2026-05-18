//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1380/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1380<F: Float>(t33743: F, t33746: F, t33750: F, t33753: F, t33755: F, t33758: F, t33760: F, t33763: F, t33770: F, t33772: F, t33774: F, t33777: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36688 = F::new(0.1351988360087076823e-6) * t33743;
    let t36689 = F::new(0.21102562238076876322e-7) * t33746;
    let t36690 = F::new(0.40021712703254065176e-7) * t33750;
    let t36691 = F::new(0.80043425406508130352e-7) * t33753;
    let t36692 = F::new(0.32826207925897363168e-8) * t33755;
    let t36693 = F::new(0.49520679385353736436e-5) * t33758;
    let t36694 = F::new(0.13259130899812740005e-6) * t33760;
    let t36695 = F::new(0.44197102999375800018e-8) * t33763;
    let t36698 = F::new(0.10567613244746075633e-6) * t33770;
    let t36699 = F::new(0.40021712703254065176e-7) * t33772;
    let t36700 = F::new(0.40094868252346065012e-6) * t33774;
    let t36701 = F::new(0.66295654499063700026e-7) * t33777;
    (t36688, t36689, t36690, t36691, t36692, t36693, t36694, t36695, t36698, t36699, t36700, t36701)
}
