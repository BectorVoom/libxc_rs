//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 871/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk871<F: Float>(t39673: F, t1570: F, t2178: F, t1557: F, t604: F, t7800: F, t605: F, t9132: F, t157: F, t40465: F, t24: F, t32905: F) -> (F, F, F, F, F, F, F, F) {
    let t40530 = F::new(140.0) / F::new(243.0) * t39673;
    let t40599 = F::new(280.0) / F::new(243.0) * t39673;
    let t40759 = t2178 * t1570;
    let t40766 = t2178 * t1557;
    let t40771 = t604 * t7800;
    let t40792 = t9132 * t605;
    let t40808 = t40465 * t157;
    let t40830 = t24 * t32905;
    (t40530, t40599, t40759, t40766, t40771, t40792, t40808, t40830)
}
