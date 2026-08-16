//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1085/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1085(t26496: f64, t786: f64, t2467: f64, t25431: f64, t26482: f64, t225: f64, t26473: f64, t2470: f64, t7406: f64, t7064: f64, t2061: f64, t2722: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26497 = t786 * t26496;
    let t26498 = t26497 * t2467;
    let t26500 = t25431 * t26482;
    let t26502 = t26473 * t225;
    let t26506 = t7406 * t2470;
    let t26508 = 0.17135234354032049604e-1_f64 * t7064 * t26506;
    let t26509 = t2061 * t2722;
    (t26497, t26498, t26500, t26502, t26506, t26508, t26509)
}
