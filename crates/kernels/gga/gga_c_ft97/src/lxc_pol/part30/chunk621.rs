//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 621/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk621<F: Float>(t28037: F, t3886: F, t28036: F, t6752: F, t684: F, t24231: F, t24455: F, t24470: F, t27466: F, t27471: F, t27473: F, t27477: F, t27481: F, t27485: F, t27745: F, t27751: F, t27755: F, t27759: F) -> (F, F, F, F, F) {
    let t28038 = t28037 * t3886;
    let t28039 = t28036 * t28038;
    let t28042 = t6752 * t684;
    let t28043 = t24231 * t28042;
    let t28057 = t27466 / F::new(18.0) + t27471 / F::new(9.0) - t27473 / F::new(27.0) - F::new(2.0) / F::new(9.0) * t27477 - F::new(2.0) * t27481 + t27485 / F::new(9.0) - t27745 / F::new(6.0) - t24455 / F::new(36.0) - t24470 / F::new(9.0) - t27751 - t27755 / F::new(9.0) - t27759 / F::new(9.0);
    (t28038, t28039, t28042, t28043, t28057)
}
