//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 991/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk991<F: Float>(t23: F, t32075: F, t1609: F, t7905: F, t1610: F, t1613: F, t1593: F, t1611: F, t533: F, t51: F, t5596: F, t1608: F, t371: F, t8052: F, t1597: F, t62: F) -> (F, F, F, F, F, F, F, F) {
    let t37429 = t23 * t32075;
    let t37452 = t1609 * t7905;
    let t37481 = t1613 * t1610 * t1609;
    let t37482 = t37481 * t1593;
    let t37487 = t1611 * t533;
    let t37550 = t5596 * t51;
    let t37551 = t1608 * t37550;
    let t37835 = t371 * t8052;
    let t37939 = t1597 * t62;
    (t37429, t37452, t37481, t37482, t37487, t37551, t37835, t37939)
}
