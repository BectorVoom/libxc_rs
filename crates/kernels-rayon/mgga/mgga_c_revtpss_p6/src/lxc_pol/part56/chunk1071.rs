//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1071/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1071(t124604: f64, t3566: f64, t1245: f64, t246: f64, t2148: f64, t3781: f64, t1276: f64, t482: f64, t372: f64, t33518: f64, t3704: f64, t33501: f64, t33508: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t124605 = t3566 * t124604;
    let t124610 = t1245 * t246;
    let t124611 = t2148 * t3781 * t124610;
    let t124612 = t482 * t1276;
    let t124613 = t372 * t124612;
    let t124619 = t33518 * t3704;
    let t124621 = t33501 * t33508;
    (t124605, t124610, t124611, t124612, t124613, t124619, t124621)
}
