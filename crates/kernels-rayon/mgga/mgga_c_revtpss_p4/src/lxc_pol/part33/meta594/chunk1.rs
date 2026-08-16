//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2012/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2012(t2681: f64, t7269: f64, t820: f64, t1416: f64, t240: f64, t25981: f64, t25987: f64, t9775: f64, t2453: f64, t4086: f64, t64: f64, t9795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94545 = t820 * t7269 * t2681;
    let t94546 = t94545 * t1416;
    let t94550 = t25981 * t240;
    let t94554 = t9775 * t25987;
    let t94564 = t2453 * t4086 * t64;
    let t94565 = t94564 * t9795;
    (t94545, t94546, t94550, t94554, t94564, t94565)
}
