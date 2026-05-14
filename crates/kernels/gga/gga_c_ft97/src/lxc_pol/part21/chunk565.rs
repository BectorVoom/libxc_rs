//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 565/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk565<F: Float>(t1948: F, t342: F, t630: F, t142: F, t1557: F, t1570: F, t520: F, t7773: F, t89: F, t128: F, t39: F, t2035: F, t2058: F, t6: F, t133: F, t542: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8764 = t342 * t630 * t1948;
    let t8766 = t142 * t1557;
    let t8774 = t142 * t1570;
    let t8796 = t89 * t7773 * t520;
    let t8811 = t128 * t39;
    let t8812 = t8811 * t2035;
    let t8832 = t2058 * t6;
    let t8833 = t133 * t8832;
    let t8838 = t542 * t8832;
    (t8764, t8766, t8774, t8796, t8811, t8812, t8832, t8833, t8838)
}
