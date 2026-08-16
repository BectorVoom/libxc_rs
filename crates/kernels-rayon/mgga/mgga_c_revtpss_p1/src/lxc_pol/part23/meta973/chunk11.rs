//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3309/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3309(t1448: f64, t6816: f64, t22852: f64, t4140: f64, t47076: f64, t48291: f64, t48293: f64, t5536: f64, t85923: f64, t85924: f64, t85925: f64, t85926: f64, t85928: f64, t85930: f64, t85932: f64) -> (f64, f64) {
    let t86771 = t6816 * t1448;
    let t86782 = 18.0_f64 * t22852 * t4140 * t5536 - t47076 + t48291 - t48293 - t85923 + t85924 - t85925 + t85926 - t85928 + t85930 - t85932;
    (t86771, t86782)
}
