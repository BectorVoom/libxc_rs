//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1380/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1380<F: Float>(t113591: F, t113595: F, t113602: F, t113610: F, t127042: F, t127731: F, t127735: F, t127739: F, t127743: F, t127745: F, t127748: F, t127752: F, t113141: F, t2665: F, t446: F, t992: F) -> (F, F) {
    let t127754 = 2.0 * t127042 - t127731 + 16.0 / 9.0 * t113591 - t113595 - t113602 - t113610 + 2.0 * t127735 + 2.0 * t127739 - t127743 - t127745 - t127748 / 2.0 - 12.0 * t127752;
    let t127759 = t446 * t2665 * t113141 * t992;
    (t127754, t127759)
}
