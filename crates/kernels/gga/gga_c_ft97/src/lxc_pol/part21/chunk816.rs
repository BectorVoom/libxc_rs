//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 816/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk816<F: Float>(t22907: F, t25605: F, t1308: F, t378: F, t108: F, t1570: F, t3188: F, t1642: F) -> (F, F, F, F, F, F) {
    let t25606 = t22907 * t25605;
    let t25609 = t378 * t1308;
    let t25610 = t108 * t1570;
    let t25611 = t25610 * t3188;
    let t25612 = t25609 * t25611;
    let t25615 = t1642 * t1308;
    (t25606, t25609, t25610, t25611, t25612, t25615)
}
