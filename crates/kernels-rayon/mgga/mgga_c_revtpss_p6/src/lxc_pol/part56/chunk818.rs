//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 818/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk818(t1204: f64, t2142: f64, t1209: f64, t26936: f64, t3801: f64, t7669: f64, t12587: f64, t2155: f64, t116: f64, t7583: f64) -> (f64, f64, f64, f64, f64) {
    let t27020 = t1204 * t2142;
    let t27025 = t1209 * t26936;
    let t27037 = t7669 * t3801;
    let t27041 = t2155 * t12587;
    let t27060 = t7583 * t116;
    (t27020, t27025, t27037, t27041, t27060)
}
