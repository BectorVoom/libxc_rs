//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1852/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1852(t7282: f64, t93139: f64, t1955: f64, t25920: f64, t4075: f64, t2028: f64, t3999: f64, t25875: f64, t4004: f64, t676: f64, t25894: f64, t25877: f64, t94382: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94701 = t93139 * t7282;
    let t94705 = t1955 * t25920 * t4075;
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94764 = t676 * t4004;
    let t94768 = t25894 * t94762;
    let t94771 = t94382 * t25877;
    (t94701, t94705, t94763, t94764, t94768, t94771)
}
