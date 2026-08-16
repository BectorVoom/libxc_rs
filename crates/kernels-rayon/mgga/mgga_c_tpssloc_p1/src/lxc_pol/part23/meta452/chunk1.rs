//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1303/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1303(t75916: f64, t75928: f64, t157: f64, t182: f64, t58057: f64, t1530: f64, t193: f64, t20756: f64, t39529: f64, t40779: f64, t40784: f64, t40790: f64, t40793: f64, t40797: f64, t75894: f64, t75895: f64, t75900: f64, t75901: f64, t870: f64) -> (f64, f64, f64, f64) {
    let t75929 = t75916 + t75928;
    let t75932 = 0.19751673498613801407e-1_f64 * t75929 * t157 * t182;
    let t75933 = 0.70178683471615754484e1_f64 * t58057;
    let t75934 = 24.0_f64 * t1530 * t193 * t20756 * t870 - t39529 - t40779 + t40784 + t40790 + t40793 + t40797 + t75894 + t75895 + t75900 - t75901 + t75932 + t75933;
    (t75929, t75932, t75933, t75934)
}
