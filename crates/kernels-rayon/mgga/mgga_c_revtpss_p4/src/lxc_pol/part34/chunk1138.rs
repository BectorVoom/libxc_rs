//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1138/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1138(t1364: f64, t27968: f64, t2022: f64, t3999: f64, t212: f64, t7910: f64, t1358: f64, t689: f64, t7925: f64, t25904: f64, t25899: f64, t116: f64, t7724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27969 = t27968 * t1364;
    let t27980 = t3999 * t2022;
    let t27985 = t212 * t7910;
    let t27986 = t27985 * t1358;
    let t27987 = t689 * t27986;
    let t27989 = t7925 * t689;
    let t27990 = t25904 * t27989;
    let t27992 = t25899 * t27989;
    let t28030 = t7724 * t116;
    (t27969, t27980, t27985, t27986, t27987, t27989, t27990, t27992, t28030)
}
