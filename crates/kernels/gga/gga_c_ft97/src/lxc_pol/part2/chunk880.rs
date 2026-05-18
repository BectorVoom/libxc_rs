//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 880/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk880<F: Float>(t13672: F, t192: F, t743: F, t248: F, t668: F, t683: F, t1152: F, t1771: F, t2345: F, t26: F, t2347: F, t3886: F, t713: F) -> (F, F, F, F, F, F) {
    let t13674 = t192 * t743 * t13672;
    let t13677 = t683 * t248 * t668;
    let t13680 = t1771 * t1152;
    let t13682 = t26 * t2345;
    let t13683 = t743 * t2347;
    let t13684 = t3886 * t713;
    (t13674, t13677, t13680, t13682, t13683, t13684)
}
