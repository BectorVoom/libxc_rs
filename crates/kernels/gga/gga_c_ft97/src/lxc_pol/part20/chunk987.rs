//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 987/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk987<F: Float>(t2380: F, t52324: F, t17818: F, t222: F, t2382: F, t226: F, t24305: F, t52358: F, t17840: F, t17841: F, t2418: F, t1609: F, t51: F, t1109: F, t213: F, t2378: F, t2395: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65688 = t52324 * t2380;
    let t65689 = t65688 * t17818;
    let t65692 = t2382 * t222;
    let t65693 = t65692 * t226;
    let t65694 = t24305 * t65693;
    let t65698 = t52358 * t2380;
    let t65699 = t65698 * t17818;
    let t65747 = t17840 * t17841 * t2418;
    let t65750 = t51 * t1609;
    let t65754 = t65750 * t213 * t1109 * t2378 * t2395;
    (t65688, t65689, t65692, t65694, t65698, t65699, t65747, t65750, t65754)
}
