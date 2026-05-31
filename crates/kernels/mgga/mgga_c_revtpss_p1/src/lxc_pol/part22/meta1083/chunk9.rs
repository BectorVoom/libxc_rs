//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3922/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3922<F: Float>(t1450: F, t22461: F, t1353: F, t21937: F, t22470: F, t22475: F, t3829: F, t4135: F, t4139: F, t47092: F, t47096: F, t47098: F, t49541: F, t5536: F, t5541: F, t74126: F, t74129: F, t74131: F, t74133: F) -> F {
    let t75389 = t22461 * t1450;
    let t75401 = F::cast_from(6.0_f64) * t1353 * t4139 * t75389 + F::cast_from(6.0_f64) * t21937 * t3829 * t5536 + F::cast_from(2.0_f64) * t22475 * t4135 * t5541 + F::cast_from(12.0_f64) * t22470 * t49541 + t47092 - t47096 - t47098 + t74126 - t74129 + t74131 - t74133;
    t75401
}
