//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 963/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk963<F: Float>(t137197: F, t137204: F, t137212: F, t137218: F, t1786: F, t7264: F, t32457: F, t487: F, t32636: F, t8392: F, t7274: F, t8417: F) -> (F, F, F, F, F, F, F, F) {
    let t137652 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t137197;
    let t137654 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t137204;
    let t137657 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t137212;
    let t137659 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t137218;
    let t137680 = t1786 * t7264;
    let t137713 = t487 * t32457;
    let t137729 = t8392 * t32636;
    let t137739 = t8417 * t7274;
    (t137652, t137654, t137657, t137659, t137680, t137713, t137729, t137739)
}
