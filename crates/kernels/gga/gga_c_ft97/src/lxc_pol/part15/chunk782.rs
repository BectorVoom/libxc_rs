//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 782/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk782<F: Float>(t1557: F, t2178: F, t604: F, t7800: F, t605: F, t9132: F, t157: F, t40465: F, t24: F, t32905: F, t2101: F, t2179: F, t40436: F, t7763: F, t143: F, t38052: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40766 = t2178 * t1557;
    let t40771 = t604 * t7800;
    let t40792 = t9132 * t605;
    let t40808 = t40465 * t157;
    let t40830 = t24 * t32905;
    let t40911 = t2101 * t2179;
    let t40926 = t40436 * t157;
    let t40931 = t604 * t7763;
    let t41002 = t38052 * t143;
    (t40766, t40771, t40792, t40808, t40830, t40911, t40926, t40931, t41002)
}
