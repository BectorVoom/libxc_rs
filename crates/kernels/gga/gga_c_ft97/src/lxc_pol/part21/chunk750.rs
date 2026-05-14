//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 750/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk750<F: Float>(t22572: F, t5569: F, t5572: F, t1630: F, t71: F, t1614: F, t47: F, t9: F) -> (F, F, F) {
    let t22574 = t5569 * t22572 * t5572;
    let t22576 = t71 * t1630;
    let t22581 = t1614 * t47;
    let t22582 = t9 * t22581;
    (t22574, t22576, t22582)
}
