//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1327/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1327<F: Float>(t28: F, t3526: F, t5842: F, t586: F, t5890: F, t105372: F, t39693: F, t446: F, t105388: F, t9073: F, t1369: F, t1637: F, t6665: F, t105345: F, t1969: F, t105349: F, t9049: F) -> (F, F, F, F, F, F, F) {
    let t105608 = t5890 * t28 * t586 * t5842 * t3526;
    let t105611 = t446 * t39693 * t105372;
    let t105614 = t446 * t9073 * t105388;
    let t105617 = t1369 * t1637 * t6665;
    let t105618 = 4.0 / 9.0 * t105617;
    let t105620 = t446 * t1969 * t105345;
    let t105623 = t446 * t9049 * t105349;
    (t105608, t105611, t105614, t105617, t105618, t105620, t105623)
}
