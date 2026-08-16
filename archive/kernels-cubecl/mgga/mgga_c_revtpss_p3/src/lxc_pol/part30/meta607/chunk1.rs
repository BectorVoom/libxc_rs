//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2071/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2071<F: Float>(t27126: F, t7003: F, t25856: F, t7732: F, t26090: F, t7898: F, t1353: F, t28198: F, t25082: F, t28197: F, t27833: F, t7239: F) -> (F, F, F, F, F) {
    let t97647 = F::cast_from(4.0_f64) * t27126 * t7003;
    let t97649 = F::cast_from(2.0_f64) * t7732 * t25856;
    let t97653 = t7898 * t26090;
    let t97654 = t28198 * t1353;
    let t97657 = F::cast_from(12.0_f64) * t25082 * t28197 * t97654;
    let t97659 = F::cast_from(6.0_f64) * t27833 * t7239;
    (t97647, t97649, t97653, t97657, t97659)
}
