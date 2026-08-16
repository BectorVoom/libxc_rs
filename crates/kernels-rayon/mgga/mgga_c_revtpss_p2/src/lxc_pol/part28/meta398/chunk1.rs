//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1505/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1505(t14609: f64, t14610: f64, t14612: f64, t14630: f64, t225: f64, t73: f64, t830: f64, t1544: f64, t2475: f64, t2394: f64, t4343: f64, t853: f64) -> (f64, f64, f64, f64, f64) {
    let t14633 = (t14609 + t14610 + t14612 + t14630) * t225;
    let t14643 = t830 * t73;
    let t14648 = t2475 * t1544;
    let t14649 = t14648 * t2394;
    let t14652 = t853 * t4343;
    (t14633, t14643, t14648, t14649, t14652)
}
