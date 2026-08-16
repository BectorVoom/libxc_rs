//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 900/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk900(t1035: f64, t1966: f64, t7556: f64, t30090: f64, t7365: f64, t1181: f64, t16325: f64, t604: f64, t7493: f64, t7353: f64, t7839: f64, t1992: f64, t3169: f64, t7585: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30727 = t1035 * t1966;
    let t30728 = t30727 * t7556;
    let t30729 = 0.56606566121287473723e-2_f64 * t30728;
    let t30730 = t30090 * t7365;
    let t30738 = t7493 * t1181 * t604 * t16325;
    let t30744 = t7839 * t7353;
    let t30748 = t7585 * t7586 * t1992 * t3169;
    (t30727, t30729, t30730, t30738, t30744, t30748)
}
