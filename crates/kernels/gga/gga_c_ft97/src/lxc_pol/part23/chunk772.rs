//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 772/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk772<F: Float>(t18712: F, t4265: F, t2881: F, t4140: F, t4139: F, t19460: F, t10479: F, t10485: F, t19465: F, t1212: F, t4311: F, t840: F, t19240: F, t319: F, t5299: F, t882: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19534 = t4265 * t18712;
    let t19535 = t2881 * t19534;
    let t19538 = t4140 * t18712;
    let t19539 = t4139 * t19538;
    let t19542 = t4140 * t19460;
    let t19543 = t10479 * t19542;
    let t19546 = t10485 * t19465;
    let t19547 = t4139 * t19546;
    let t19551 = t840 * t4311 * t1212;
    let t19555 = t840 * t319 * t19240;
    let t19559 = t840 * t882 * t5299;
    (t19534, t19535, t19538, t19539, t19542, t19543, t19546, t19547, t19551, t19555, t19559)
}
