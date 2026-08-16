//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 973/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk973(t1203: f64, t482: f64, t372: f64, t371: f64, t33468: f64, t487: f64, t33494: f64) -> (f64, f64, f64, f64) {
    let t33496 = t482 * t1203;
    let t33497 = t372 * t33496;
    let t33498 = t371 * t33497;
    let t33501 = t33468 * t487;
    let t33502 = t33501 * t33494;
    (t33496, t33498, t33501, t33502)
}
