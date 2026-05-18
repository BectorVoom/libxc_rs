//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 945/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk945<F: Float>(t11543: F, t11546: F, t116: F, t655: F, t3163: F, t3691: F, t3696: F, t3703: F, t424: F, t134: F, t3698: F, t3702: F) -> (F, F, F, F, F, F, F) {
    let t11547 = t11543 * t11546;
    let t11549 = t116 * t655;
    let t11550 = t11549 * t11546;
    let t11552 = t3691 * t3163;
    let t11555 = t424 * t3696 * t3703;
    let t11557 = t3698 * t134;
    let t11558 = t11557 * t3702;
    (t11547, t11549, t11550, t11552, t11555, t11557, t11558)
}
