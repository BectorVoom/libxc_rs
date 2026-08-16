//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 705/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk705(t213: f64, t5744: f64, t4086: f64, t2242: f64, t38: f64, t1925: f64) -> (f64, f64, f64, f64) {
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t6954 = t2242 * t38;
    let t6957 = t38 * t1925;
    (t5745, t5755, t6954, t6957)
}
