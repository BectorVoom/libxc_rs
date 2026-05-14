//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1219/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1219<F: Float>(t1655: F, t26654: F, t1657: F, t18402: F, t2169: F, t2209: F, t233: F, t27155: F, t27734: F, t27746: F, t2802: F, t3703: F, t7673: F, t8024: F, t8121: F, t92344: F, t92351: F, t92356: F, t92360: F, t92368: F, t92375: F) -> (F,) {
    let t97601 = t1655 * t26654;
    let t97602 = t92344 - t27155 * t8024 / 8.0 - t233 * t2802 * t8121 / 16.0 - t92351 - t233 * t18402 * t2209 / 16.0 + t92356 - t92360 + t7673 * t27734 / 8.0 + t92368 - t92375 - t2169 * t1657 * t3703 / 16.0 + t7673 * t27746 / 8.0 + t97601;
    (t97602,)
}
