//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1066/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1066<F: Float>(t135: F, t1992: F, t2035: F, t39: F, t538: F, t3056: F, t71: F, t526: F, t597: F, t12939: F, t604: F, t2178: F, t3539: F, t167: F, t40436: F, t605: F, t9224: F) -> (F, F, F, F, F, F, F, F) {
    let t48678 = t1992 * t135;
    let t48841 = t538 * t39 * t2035;
    let t49004 = t71 * t3056;
    let t49414 = t526 * t597;
    let t49553 = t12939 * t604;
    let t49562 = t3539 * t2178;
    let t49579 = t40436 * t167;
    let t49583 = t9224 * t605;
    (t48678, t48841, t49004, t49414, t49553, t49562, t49579, t49583)
}
