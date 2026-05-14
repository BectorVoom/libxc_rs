//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 915/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk915<F: Float>(t144: F, t26591: F, t26593: F, t26584: F, t1882: F, t6705: F, t12680: F, t5943: F, t23455: F, t3478: F, t13140: F, t143: F, t23: F, t157: F, t1384: F, t9438: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27313 = t144 * t26591;
    let t27316 = t144 * t26593;
    let t27320 = t144 * t26584;
    let t27324 = t1882 * t6705;
    let t27326 = t12680 * t5943;
    let t27329 = t23455 * t3478;
    let t27330 = t13140 * t27329;
    let t27333 = t23 * t143;
    let t27334 = t27333 * t157;
    let t27335 = t9438 * t1384;
    (t27313, t27316, t27320, t27324, t27326, t27329, t27330, t27333, t27334, t27335)
}
