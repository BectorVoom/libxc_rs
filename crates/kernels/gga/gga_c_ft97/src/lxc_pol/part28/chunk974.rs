//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 974/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk974<F: Float>(t140: F, t147273: F, t147319: F, t147357: F, t147402: F, t147448: F, t147492: F, t147541: F, t147586: F, t138537: F, t6584: F, t32748: F, t6580: F, t1349: F, t138551: F, t138557: F, t138560: F, t138652: F, t138655: F, t165: F, t23413: F, t26515: F, t26581: F, t27422: F, t28: F, t35007: F, t35234: F, t378: F, t525: F, t5772: F, t5845: F, t7309: F, t7313: F, t7315: F) -> (F, F) {
    let t141 = 0.1e-59 < t140;
    let t147590 = piecewise3(t141, t147273 + t147319 + t147357 + t147402 + t147448 + t147492 + t147541 + t147586, 0.0);
    let t147602 = t138537 * t6584;
    let t147604 = t6580 * t32748;
    let t147614 = -t138551 / 18.0 + t35007 * t5845 / 6.0 + t1349 * t28 * t525 * t147590 * t165 / 6.0 + t138557 / 9.0 - 2.0 / 9.0 * t5772 * t378 * t7313 * t27422 - t138560 / 18.0 + t147602 / 54.0 + t147604 / 9.0 - t23413 * t35234 / 9.0 + t7309 * t26515 / 6.0 - t138652 / 18.0 - t138655 / 9.0 - t26581 * t7315 / 3.0;
    (t147590, t147614)
}
