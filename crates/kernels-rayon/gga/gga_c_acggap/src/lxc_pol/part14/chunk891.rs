//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 891/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk891(t1035: f64, t1966: f64, t7556: f64, t30090: f64, t7365: f64, t1103: f64, t7736: f64, t1089: f64, t429: f64, t7553: f64, t7554: f64, t1998: f64, t3756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30727 = t1035 * t1966;
    let t30728 = t30727 * t7556;
    let t30729 = 0.56606566121287473723e-2_f64 * t30728;
    let t30730 = t30090 * t7365;
    let t30769 = t7736 * t1103;
    let t30773 = t7553 * t1089 * t429 * t7554;
    let t30775 = t1998 * t3756;
    (t30727, t30729, t30730, t30769, t30773, t30775)
}
