//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 721/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk721<F: Float>(t2609: F, t9787: F, t2360: F, t761: F, t2349: F, t766: F, t2606: F, t713: F, t3885: F, t2599: F, t2344: F, t675: F) -> (F, F, F, F, F, F, F, F) {
    let t9788 = t9787 * t2609;
    let t9791 = t761 * t2360;
    let t9792 = t2349 * t766;
    let t9793 = t9791 * t9792;
    let t9794 = t2606 * t9793;
    let t9797 = t2349 * t713;
    let t9798 = t3885 * t9797;
    let t9799 = t2599 * t9798;
    let t9802 = t2344 * t675;
    (t9788, t9792, t9793, t9794, t9797, t9798, t9799, t9802)
}
