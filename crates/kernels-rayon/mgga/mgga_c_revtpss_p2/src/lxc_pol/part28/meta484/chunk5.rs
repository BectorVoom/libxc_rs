//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1842/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1842(t265: f64, t393: f64, t1100: f64, t1102: f64, t198: f64, t25705: f64, t25709: f64, t25713: f64, t25743: f64, t3329: f64, t3333: f64, t336: f64, t5023: f64, t7181: f64) -> f64 {
    let t394 = t265 < t393;
    let t25744 = piecewise3(t394, t1102 * t198 * t25705 * t336 - 2.0_f64 * t1100 * t25709 * t5023 + 2.0_f64 * t25713 * t3333 * t5023 - t3329 * t5023 * t7181, t25743);
    t25744
}
