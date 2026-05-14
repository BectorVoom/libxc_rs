//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 848/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk848<F: Float>(t11539: F, t5059: F, t11537: F, t116: F, t641: F, t1908: F, t198: F, t3137: F, t655: F, t3163: F, t3691: F, t3696: F, t3703: F, t424: F, t134: F, t3698: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11540 = t11539 * t5059;
    let t11541 = t11537 * t11540;
    let t11543 = t116 * t641;
    let t11546 = t3137 * t198 * t1908 * t5059;
    let t11547 = t11543 * t11546;
    let t11549 = t116 * t655;
    let t11550 = t11549 * t11546;
    let t11552 = t3691 * t3163;
    let t11555 = t424 * t3696 * t3703;
    let t11557 = t3698 * t134;
    (t11540, t11541, t11543, t11546, t11547, t11549, t11550, t11552, t11555, t11557)
}
