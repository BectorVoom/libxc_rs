//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3286/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3286<F: Float>(t162: F, t4403: F, t50903: F, t50089: F, t14331: F, t13312: F, t4401: F, t4402: F, t50880: F, t50883: F, t50888: F, t2609: F, t5944: F) -> (F, F, F, F, F, F, F) {
    let t62290 = F::cast_from(48.0_f64) * t50903 * t162 * t4403;
    let t62291 = t50089 * t162;
    let t62293 = F::cast_from(48.0_f64) * t62291 * t14331;
    let t62296 = F::cast_from(24.0_f64) * t4401 * t4402 * t13312;
    let t62297 = F::cast_from(48.0_f64) * t50880;
    let t62298 = F::cast_from(0.46785788981077169656e1_f64) * t50883;
    let t62299 = F::cast_from(0.70178683471615754484e1_f64) * t50888;
    let t62300 = t5944 * t2609;
    (t62290, t62293, t62296, t62297, t62298, t62299, t62300)
}
