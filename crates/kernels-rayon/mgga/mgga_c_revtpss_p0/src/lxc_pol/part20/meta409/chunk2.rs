//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1515/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1515(t3046: f64, t3316: f64, t4891: f64, t11923: f64, t11933: f64, t41229: f64, t41241: f64, t41243: f64, t41449: f64, t41451: f64, t41453: f64, t41455: f64, t41459: f64, t41468: f64, t41472: f64, t41476: f64) -> (f64, f64, f64) {
    let t42830 = t3046 * t3316 * t4891;
    let t42833 = t11933 * t11923;
    let t42846 = t41229 - t41241 - t41243 - t41449 + t41451 - t41453 - t41455 + t41459 + t41468 - t41472 - t41476;
    (t42830, t42833, t42846)
}
