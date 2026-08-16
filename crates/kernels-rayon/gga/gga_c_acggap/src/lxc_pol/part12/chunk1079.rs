//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1079/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1079(t1967: f64, t8549: f64, t30219: f64, t8515: f64, t4680: f64, t7575: f64, t8514: f64, t1181: f64, t4930: f64, t604: f64, t4550: f64, t1345: f64, t1992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35210 = t1967 * t8549;
    let t35212 = t30219 * t8515;
    let t35215 = t7575 * t4680 * t8514;
    let t35219 = t7575 * t1181 * t604 * t4930;
    let t35223 = t7575 * t1181 * t604 * t4550;
    let t35225 = t1992 * t1345;
    (t35210, t35212, t35215, t35219, t35223, t35225)
}
