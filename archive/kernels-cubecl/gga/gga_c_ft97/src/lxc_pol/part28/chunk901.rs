//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 901/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk901<F: Float>(t1609: F, t1610: F, t1613: F, t1593: F, t1597: F, t62: F, t66: F, t11240: F, t371: F, t19: F, t7: F, t11: F, t1690: F) -> (F, F, F, F, F, F, F) {
    let t37481 = t1613 * t1610 * t1609;
    let t37482 = t37481 * t1593;
    let t37939 = t1597 * t62;
    let t37940 = t37939 * t66;
    let t37985 = t371 * t11240;
    let t37991 = t7 * t19;
    let t38176 = t1690 * t11;
    (t37481, t37482, t37939, t37940, t37985, t37991, t38176)
}
