//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 991/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk991(t2132: f64, t322: f64, t7896: f64, t8422: f64, t556: f64, t943: f64, t944: f64, t880: f64, t9062: f64, t157: f64, t929: f64, t1960: f64, t5368: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33635 = t7896 * t2132 * t8422 * t322;
    let t33643 = t556 * t943;
    let t33644 = t33643 * t944;
    let t33648 = t9062 * t880;
    let t33651 = t556 * t929 * t157;
    let t33656 = t1960 * t5368;
    (t33635, t33643, t33644, t33648, t33651, t33656)
}
