//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2427/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2427<F: Float>(t273: F, t270: F, t276: F, t39484: F, t11318: F, t698: F, t9303: F, t931: F, t11571: F, t300: F, t2922: F, t275: F) -> (F, F, F, F, F, F) {
    let t41382 = F::powf(t273, -F::new(0.25e1));
    let t41401 = F::new(1.0) / t276 / t39484 / t270 / F::new(96.0);
    let t41406 = t698 * t11318;
    let t41441 = t9303 * t931;
    let t41491 = t300 * t11571;
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    (t41382, t41401, t41406, t41441, t41491, t41499)
}
