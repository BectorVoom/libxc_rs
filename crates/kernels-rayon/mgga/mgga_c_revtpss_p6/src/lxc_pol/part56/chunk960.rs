//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 960/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk960(t1248: f64, t1250: f64, t494: f64, t3720: f64, t12915: f64, t247: f64, t8926: f64, t370: f64, t8924: f64) -> (f64, f64, f64, f64, f64) {
    let t33416 = t494 * t1248 * t1250;
    let t33417 = t3720 * t33416;
    let t33421 = t247 * t12915 * t494;
    let t33423 = 0.18822977838986977999e-3_f64 * t8926 * t33421;
    let t33424 = t8924 * t370;
    (t33416, t33417, t33421, t33423, t33424)
}
