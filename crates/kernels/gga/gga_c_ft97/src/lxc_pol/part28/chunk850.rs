//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 850/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk850<F: Float>(t137204: F, t137212: F, t137218: F, t1786: F, t7264: F, t32457: F, t487: F, t32636: F, t8392: F, t7274: F, t8417: F, t32594: F, t1851: F, t7281: F, t1882: F, t32475: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t137654 = 8.0 / 9.0 * t137204;
    let t137657 = 4.0 / 9.0 * t137212;
    let t137659 = 2.0 / 9.0 * t137218;
    let t137680 = t1786 * t7264;
    let t137713 = t487 * t32457;
    let t137729 = t8392 * t32636;
    let t137739 = t8417 * t7274;
    let t137768 = t8392 * t32594;
    let t137797 = t1851 * t7281;
    let t137802 = t1882 * t32475;
    (t137654, t137657, t137659, t137680, t137713, t137729, t137739, t137768, t137797, t137802)
}
