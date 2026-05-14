//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1031/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1031<F: Float>(t1300: F, t93191: F, t3076: F, t32167: F, t5585: F, t172: F, t5589: F, t72: F, t1602: F, t92685: F, t70: F, t1642: F, t1800: F, t378: F, t8270: F, t1766: F, t1900: F, t6: F, t91: F) -> (F, F, F, F, F, F, F, F) {
    let t93192 = t1300 * t93191;
    let t93229 = t3076 * t32167 * t5585;
    let t93252 = t5589 * t172;
    let t93253 = t93252 * t72;
    let t93268 = t1602 * t92685;
    let t93324 = t93252 * t70;
    let t93351 = t1642 * t1800;
    let t93355 = t378 * t8270;
    let t93378 = t91 * t1766 * t6 * t1900;
    (t93192, t93229, t93253, t93268, t93324, t93351, t93355, t93378)
}
