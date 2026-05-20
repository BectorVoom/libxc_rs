//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2629/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2629<F: Float>(t48333: F, t5571: F, t9419: F, t40076: F, t40079: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F, t48322: F, t48323: F, t48325: F, t48327: F, t48328: F, t48329: F, t48330: F, t48332: F) -> (F, F, F) {
    let t48334 = F::new(36.0) * t48333;
    let t48335 = t5571 * t9419;
    let t48336 = F::cast_from(0.10389515463408878255e3_f64) * t48335;
    let t48337 = t47131 + t48322 - t48323 - t47138 - t47140 + t47142 - t48325 - t48327 + t40076 - t40079 - t48328 - t48329 + t48330 - t48332 + t47152 + t48334 + t48336;
    (t48334, t48336, t48337)
}
