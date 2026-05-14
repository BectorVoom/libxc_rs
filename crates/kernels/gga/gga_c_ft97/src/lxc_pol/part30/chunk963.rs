//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 963/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk963<F: Float>(t24980: F, t28511: F, t2862: F, t6318: F, t33961: F, t4255: F, t6317: F, t99559: F, t6260: F, t6334: F, t992: F, t24976: F, t152673: F, t152678: F, t99391: F, t33847: F, t4162: F, t44280: F) -> (F, F, F, F, F, F, F, F) {
    let t152715 = t24980 * t2862 * t6318 * t28511;
    let t152717 = t33961 * t4255;
    let t152719 = t6317 * t99559 * t152717;
    let t152722 = t6334 * t992 * t6260;
    let t152724 = t6317 * t24976 * t152722;
    let t152727 = t6317 * t24976 * t152673;
    let t152730 = t6317 * t99391 * t152678;
    let t152734 = t6317 * t44280 * t33847 * t4162;
    (t152715, t152717, t152719, t152722, t152724, t152727, t152730, t152734)
}
