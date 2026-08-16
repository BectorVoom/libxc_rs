//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1238/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1238(t2037: f64, t8249: f64, t1913: f64, t8776: f64, t34468: f64, t575: f64, t34490: f64, t571: f64, t127439: f64, t127442: f64, t127443: f64, t127447: f64, t127449: f64, t127453: f64, t127455: f64, t127459: f64, t127462: f64, t1461: f64, t32377: f64, t34477: f64) -> (f64, f64, f64, f64, f64) {
    let t129530 = t2037 * t8249;
    let t129531 = t1913 * t8776;
    let t129533 = t34468 * t575;
    let t129534 = t571 * t34490;
    let t129540 = 3.0_f64 * t1461 * t34477 + 3.0_f64 * t127439 + t127442 + 6.0_f64 * t127443 + t127447 + t127449 + t127453 + t127455 + t127459 + t127462 + t32377;
    (t129530, t129531, t129533, t129534, t129540)
}
