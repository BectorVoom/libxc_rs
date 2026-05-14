//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 790/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk790<F: Float>(t342: F, t657: F, t8639: F, t240: F, t9570: F, t762: F, t9895: F, t2492: F, t2568: F, t255: F, t42109: F, t9802: F, t42163: F, t761: F, t241: F, t41752: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42293 = 5.0 / 54.0 * t342 * t8639 * t657;
    let t42307 = t240 * t9570;
    let t42334 = t9895 * t762;
    let t42339 = t2492 * t2568;
    let t42350 = t42109 * t255;
    let t42362 = t9802 * t762;
    let t42409 = t42163 * t255;
    let t42416 = t761 * t9570;
    let t42469 = t41752 * t241;
    (t42293, t42307, t42334, t42339, t42350, t42362, t42409, t42416, t42469)
}
