//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1780/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1780<F: Float>(t3167: F, t7120: F, t1033: F, t3173: F, t7122: F, t1007: F, t7106: F, t1968: F, t3080: F, t7105: F, t800: F) -> (F, F, F, F, F, F) {
    let t25525 = t7120 * t3167;
    let t25526 = t1033 * t25525;
    let t25529 = t7122 * t3173;
    let t25535 = t7106 * t1007;
    let t25538 = t1968 * t3080 / F::new(432.0);
    let t25539 = t7105 * t800;
    (t25525, t25526, t25529, t25535, t25538, t25539)
}
