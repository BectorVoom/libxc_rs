//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1281/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1281<F: Float>(t124495: F, t124507: F, t124515: F, t124528: F, t124536: F, t124547: F, t124561: F, t124568: F, t124583: F, t124593: F, t124602: F, t124610: F, t124621: F, t124632: F, t124646: F, t124653: F, t762: F) -> (F,) {
    let t124658 = t762 * (t124495 + t124507 + t124515 + t124528 + t124536 + t124547 + t124561 + t124568 + t124583 + t124593 + t124602 + t124610 + t124621 + t124632 + t124646 + t124653);
    (t124658,)
}
