//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 604/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk604(t1250: f64, t3588: f64, t482: f64, t1042: f64, t3140: f64, t460: f64, t1242: f64, t472: f64) -> (f64, f64, f64, f64) {
    let t3590 = t482 * t3588 * t1250;
    let t3591 = t1042 * t3590;
    let t3594 = t460 * t3140;
    let t3596 = 1.0_f64 / t1242 / t472;
    (t3590, t3591, t3594, t3596)
}
