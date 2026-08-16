//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1480/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1480(t2741: f64, t6019: f64, t5966: f64, t775: f64, t10698: f64, t828: f64, t1544: f64, t4343: f64) -> (f64, f64, f64, f64) {
    let t18491 = t2741 * t6019;
    let t18493 = t5966 * t775;
    let t18495 = t10698 * t828 * t18493;
    let t18498 = t1544 * t4343;
    (t18491, t18493, t18495, t18498)
}
