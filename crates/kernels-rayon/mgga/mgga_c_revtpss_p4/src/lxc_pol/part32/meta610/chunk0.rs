//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1949/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1949(t1711: f64, t4343: f64, t106561: f64, t27799: f64, t105923: f64, t25759: f64, t11064: f64, t27384: f64, t106533: f64, t100987: f64, t18875: f64, t4433: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t107901 = t1711 * t4343;
    let t107908 = t27799 * t106561;
    let t107919 = t25759 * t105923;
    let t107923 = t11064 * t1711;
    let t107924 = t107923 * t27384;
    let t107927 = t25759 * t106533;
    let t107930 = t100987 * t18875;
    let t107934 = t892 * t1711 * t4433;
    (t107901, t107908, t107919, t107924, t107927, t107930, t107934)
}
