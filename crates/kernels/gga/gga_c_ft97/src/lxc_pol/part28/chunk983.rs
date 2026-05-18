//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 983/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk983<F: Float>(t1349: F, t32710: F, t376: F, t32699: F, t5766: F, t33119: F, t604: F, t32695: F, t23405: F, t32724: F, t23608: F, t631: F) -> (F, F, F, F, F, F) {
    let t139159 = t1349 * t376 * t32710;
    let t139171 = t5766 * t32699;
    let t139173 = t33119 * t604;
    let t139179 = t1349 * t376 * t32695;
    let t139192 = t23405 * t32724;
    let t139212 = t23608 * t631;
    (t139159, t139171, t139173, t139179, t139192, t139212)
}
