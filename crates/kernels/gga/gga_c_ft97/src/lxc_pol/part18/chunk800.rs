//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 800/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk800<F: Float>(t22605: F, t5540: F, t1647: F, t2258: F, t5579: F, t5568: F, t7837: F) -> (F, F, F, F) {
    let t22606 = t5540 * t22605;
    let t22609 = t2258 * t1647;
    let t22610 = t5579 * t22609;
    let t22613 = t7837 * t5568;
    (t22606, t22609, t22610, t22613)
}
