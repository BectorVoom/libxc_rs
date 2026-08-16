//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 843/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk843(t225: f64, t5710: f64, t1892: f64, t213: f64, t1357: f64, t1904: f64, t689: f64, t1903: f64, t72: f64, t686: f64, t3915: f64, t1444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5711 = t5710 * t225;
    let t5715 = t213 * t1892;
    let t5718 = t1357 * t1904;
    let t5719 = t689 * t5718;
    let t5721 = t1903 * t72;
    let t5722 = t5721 * t686;
    let t5723 = t3915 * t5722;
    let t5727 = t1903 * t1444;
    (t5711, t5715, t5718, t5719, t5721, t5722, t5723, t5727)
}
