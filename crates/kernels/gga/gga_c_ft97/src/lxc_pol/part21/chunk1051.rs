//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1051/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1051<F: Float>(t100522: F, t25708: F, t1608: F, t92439: F, t1630: F, t5546: F, t22583: F, t25688: F, t92466: F, t22576: F, t420: F, t358: F, t938: F, t5569: F, t6441: F, t93003: F) -> (F, F, F, F, F, F, F) {
    let t100524 = 0.56749874115226337448e-2 * t25708 * t100522;
    let t100540 = t1608 * t92439;
    let t100541 = t5546 * t1630;
    let t100554 = t22583 * t92466 * t25688;
    let t100556 = t420 * t22576;
    let t100580 = t938 * t358;
    let t100610 = t5569 * t93003 * t6441;
    (t100524, t100540, t100541, t100554, t100556, t100580, t100610)
}
