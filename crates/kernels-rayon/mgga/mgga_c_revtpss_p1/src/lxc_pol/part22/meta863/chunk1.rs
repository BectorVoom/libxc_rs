//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3016/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3016(t40834: f64, t50613: f64, t854: f64, t14587: f64, t2735: f64, t40798: f64, t826: f64, t10777: f64, t10779: f64, t2749: f64, t50412: f64, t14686: f64, t837: f64) -> (f64, f64, f64, f64) {
    let t50615 = t40834 * t854 * t50613;
    let t50619 = t2735 * t40798 * t826 * t14587;
    let t50628 = t10777 * t10779 * t50412 * t2749;
    let t50632 = t10777 * t14686 * t50412 * t837;
    (t50615, t50619, t50628, t50632)
}
