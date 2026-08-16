//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 743/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk743(t3140: f64, t3268: f64, t1035: f64, t8507: f64, t1078: f64) -> (f64, f64, f64, f64) {
    let t8515 = t3140 * t3268;
    let t8517 = t8515 * t1035 * t8507;
    let t8520 = t3140 * t1078;
    let t8521 = t8520 * t1035;
    (t8515, t8517, t8520, t8521)
}
