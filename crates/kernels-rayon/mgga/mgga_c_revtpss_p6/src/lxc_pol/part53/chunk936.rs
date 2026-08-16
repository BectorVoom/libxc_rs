//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 936/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk936(t212: f64, t7910: f64, t1358: f64, t689: f64, t7925: f64, t25904: f64, t25899: f64, t2022: f64, t5774: f64, t7296: f64, t1955: f64, t5710: f64) -> (f64, f64, f64, f64, f64) {
    let t27985 = t212 * t7910;
    let t27986 = t27985 * t1358;
    let t27987 = t689 * t27986;
    let t27989 = t7925 * t689;
    let t27990 = t25904 * t27989;
    let t27992 = t25899 * t27989;
    let t28002 = t2022 * t5774;
    let t28003 = t7296 * t28002;
    let t28008 = t1955 * t5710;
    (t27987, t27990, t27992, t28003, t28008)
}
