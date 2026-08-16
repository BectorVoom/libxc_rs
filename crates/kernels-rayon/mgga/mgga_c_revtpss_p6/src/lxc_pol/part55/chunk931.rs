//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 931/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk931(t27879: f64, t27907: f64, t27984: f64, t28017: f64, t532: f64, t1450: f64, t2014: f64, t1513: f64, t25823: f64, t665: f64, t25826: f64, t4287: f64, t6998: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28019 = t27879 + t27907 + t27984 + t28017;
    let t28020 = t532 * t28019;
    let t28021 = t28020 * t1450;
    let t28022 = t2014 * t28021;
    let t28034 = t25823 * t1513;
    let t28036 = t1513 * t665;
    let t28037 = t25826 * t28036;
    let t28039 = t6998 * t4287;
    (t28019, t28021, t28022, t28034, t28037, t28039)
}
