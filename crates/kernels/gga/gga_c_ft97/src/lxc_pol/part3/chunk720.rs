//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 720/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk720<F: Float>(t16698: F, t446: F, t12307: F, t12309: F, t12311: F, t12328: F, t12357: F, t12359: F, t12366: F, t12913: F, t16668: F, t16673: F, t16677: F, t16679: F, t16684: F, t16689: F, t16692: F, t16696: F, t8796: F, t9065: F, t9072: F) -> (F, F) {
    let t16699 = t446 * t16698;
    let t16704 = -2.0 / 9.0 * t16668 - 2.0 / 9.0 * t16673 + 2.0 / 27.0 * t16677 - t16679 / 27.0 + t16684 / 18.0 - t16689 / 9.0 + 4.0 / 9.0 * t16692 + t16696 / 18.0 + t16699 / 9.0 - t12307 - t12309 + t12311 - t12328 - 2.0 / 27.0 * t9065 - 2.0 / 81.0 * t8796 - t12357 + 2.0 / 27.0 * t12359 - t12913 - t9072 + t12366;
    (t16699, t16704)
}
