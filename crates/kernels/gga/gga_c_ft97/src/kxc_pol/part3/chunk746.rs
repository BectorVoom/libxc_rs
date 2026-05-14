//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 746/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk746<F: Float>(t17239: F, t590: F, t91: F, t3491: F, t3526: F, t16710: F, t16714: F, t16717: F, t16721: F, t16724: F, t16727: F, t16730: F, t16734: F, t17237: F, t16745: F, t16748: F) -> (F, F, F, F, F) {
    let t17241 = t91 * t17239 * t590;
    let t17244 = t91 * t3491 * t3526;
    let t17246 = 4.0 / 3.0 * t16710 - 2.0 / 3.0 * t16714 - 2.0 * t16717 + 2.0 / 9.0 * t16721 + 4.0 / 3.0 * t16724 - 10.0 / 27.0 * t16727 - 8.0 / 9.0 * t16730 + 2.0 / 3.0 * t16734 - t17237 / 4.0 + 3.0 / 8.0 * t17241 - t17244 / 2.0;
    let t17249 = t16745 / 9.0;
    let t17250 = 2.0 / 9.0 * t16748;
    (t17241, t17244, t17246, t17249, t17250)
}
