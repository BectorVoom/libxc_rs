//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 972/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk972<F: Float>(t1349: F, t32997: F, t376: F, t32737: F, t32871: F, t458: F, t7308: F, t5775: F, t32742: F, t24087: F, t7309: F, t32748: F, t5766: F) -> (F, F, F, F, F, F, F, F) {
    let t138521 = t1349 * t376 * t32997;
    let t138524 = t1349 * t376 * t32737;
    let t138533 = t1349 * t376 * t32871;
    let t138537 = t7308 * t458;
    let t138538 = t138537 * t5775;
    let t138549 = t1349 * t376 * t32742;
    let t138551 = t7309 * t24087;
    let t138557 = t5766 * t32748;
    (t138521, t138524, t138533, t138537, t138538, t138549, t138551, t138557)
}
