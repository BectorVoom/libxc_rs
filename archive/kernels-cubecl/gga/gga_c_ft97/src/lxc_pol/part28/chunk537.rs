//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 537/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk537<F: Float>(t1614: F, t22563: F, t7837: F, t410: F, t70: F, t414: F, t5569: F, t5572: F, t47: F, t9: F, t1624: F, t373: F, t422: F) -> (F, F, F, F, F, F, F) {
    let t22564 = t22563 * t1614;
    let t22565 = t7837 * t22564;
    let t22568 = t410 * t70;
    let t22572 = t414 * t70;
    let t22574 = t5569 * t22572 * t5572;
    let t22581 = t1614 * t47;
    let t22582 = t9 * t22581;
    let t22583 = t1624 * t22582;
    let t22584 = t422 * t373;
    (t22565, t22568, t22572, t22574, t22581, t22583, t22584)
}
