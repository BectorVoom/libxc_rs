//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 959/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk959(t7839: f64, t8739: f64, t1983: f64, t30692: f64, t5720: f64, t7586: f64, t8779: f64, t1089: f64, t535: f64, t7553: f64, t7554: f64, t7637: f64, t8491: f64) -> (f64, f64, f64, f64, f64) {
    let t33986 = t7839 * t8739;
    let t33994 = t30692 * t7586 * t1983 * t5720;
    let t33996 = t7839 * t8779;
    let t34009 = t7553 * t1089 * t535 * t7554;
    let t34011 = t7637 * t8491;
    (t33986, t33994, t33996, t34009, t34011)
}
