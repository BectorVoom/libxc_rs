//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1332/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1332(t2028: f64, t3999: f64, t25875: f64, t4004: f64, t676: f64, t25880: f64, t25894: f64, t25877: f64, t94382: f64, t94590: f64, t25950: f64, t26050: f64) -> (f64, f64, f64, f64) {
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94764 = t676 * t4004;
    let t94765 = t25880 * t94764;
    let t94766 = t94763 * t94765;
    let t94768 = t25894 * t94762;
    let t94769 = t94768 * t94765;
    let t94771 = t94382 * t25877;
    let t94772 = t94771 * t94590;
    let t94774 = t25950 * t26050;
    (t94766, t94769, t94772, t94774)
}
