//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1187/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1187(t644: f64, t6977: f64, t25113: f64, t77: f64, t1927: f64, t2315: f64, t2247: f64, t2259: f64, t843: f64, t10406: f64, t76: f64, t38: f64, t45955: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92576 = t6977 * t644;
    let t92581 = t77 * t25113 * t644;
    let t92584 = t1927 * t2315;
    let t92588 = t2247 * t2259;
    let t92612 = 1232.0_f64 / 27.0_f64 * t843;
    let t92628 = t76 * t10406;
    let t92632 = t45955 * t38;
    (t92576, t92581, t92584, t92588, t92612, t92628, t92632)
}
