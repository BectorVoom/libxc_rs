//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 722/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk722(t409: f64, t7685: f64, t368: f64, t4352: f64, t7656: f64, t598: f64, t1089: f64, t3300: f64, t7679: f64, t2100: f64, t7676: f64, t1988: f64, t2092: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7686 = t7685 * t409;
    let t7689 = t4352 * t368 * t7656;
    let t7690 = t598 * t7689;
    let t7693 = t1089 * t3300 * t7679;
    let t7694 = t598 * t7693;
    let t7696 = t7676 * t2100;
    let t7698 = t1988 * t2092;
    (t7686, t7689, t7690, t7693, t7694, t7696, t7698)
}
