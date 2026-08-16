//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 485/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk485(t474: f64, t479: f64, t3089: f64, t1285: f64, t1264: f64, t828: f64, t1248: f64, t73: f64, t1121: f64, t471: f64, t606: f64, t126: f64, t1263: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3623 = t474 * t479;
    let t3624 = t3623 * t3089;
    let t3625 = t1285 * t3624;
    let t3626 = t828 * t1264;
    let t3627 = t1248 * t73;
    let t3628 = t471 * t1121;
    let t3629 = t3628 * t606;
    let t3634 = t126 * t1263;
    (t3623, t3624, t3625, t3626, t3627, t3629, t3634)
}
