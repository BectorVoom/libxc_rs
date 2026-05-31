//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2083/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2083<F: Float>(t26090: F, t7898: F, t1353: F, t28198: F, t25082: F, t28197: F, t27833: F, t7239: F, t28177: F, t7235: F, t28056: F, t4254: F) -> (F, F, F, F, F) {
    let t97653 = t7898 * t26090;
    let t97654 = t28198 * t1353;
    let t97657 = F::cast_from(12.0_f64) * t25082 * t28197 * t97654;
    let t97659 = F::cast_from(6.0_f64) * t27833 * t7239;
    let t97661 = F::cast_from(6.0_f64) * t7235 * t28177;
    let t97663 = F::cast_from(4.0_f64) * t4254 * t28056;
    (t97653, t97657, t97659, t97661, t97663)
}
