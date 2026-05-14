//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 667/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk667<F: Float>(t140: F, t20651: F, t550: F, t133: F, t2001: F, t20576: F, t20578: F, t20580: F, t20583: F, t20586: F, t20632: F, t20636: F, t3392: F, t3393: F, t4710: F) -> (F,) {
    let t141 = 0.1e-59 < t140;
    let t20652 = t550 * t20651;
    let t20653 = t133 * t20652;
    let t20655 = piecewise3(t141, 6.0 * t3392 * t3393 * t4710 + 12.0 * t2001 * t20580 - 6.0 * t2001 * t20583 - 6.0 * t2001 * t20586 + 6.0 * t20576 - 6.0 * t20578 + 2.0 * t20632 - 6.0 * t20636 - t20653, 0.0);
    (t20655,)
}
