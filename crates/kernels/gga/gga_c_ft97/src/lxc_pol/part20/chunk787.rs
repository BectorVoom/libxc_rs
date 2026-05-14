//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 787/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk787<F: Float>(t24569: F, t2608: F, t14175: F, t2469: F, t6166: F, t729: F, t6187: F, t713: F, t762: F, t1449: F, t2459: F, t242: F, t24421: F, t24565: F, t6061: F, t773: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24760 = t24569 * t2608;
    let t24761 = t14175 * t24760;
    let t24765 = t729 * t2469 * t6166;
    let t24768 = t6187 * t713;
    let t24770 = t729 * t762 * t24768;
    let t24773 = t1449 * t2459;
    let t24775 = t729 * t762 * t24773;
    let t24778 = t242 * t24421;
    let t24781 = t242 * t24565;
    let t24785 = t729 * t773 * t6061;
    (t24760, t24761, t24765, t24768, t24770, t24773, t24775, t24778, t24781, t24785)
}
