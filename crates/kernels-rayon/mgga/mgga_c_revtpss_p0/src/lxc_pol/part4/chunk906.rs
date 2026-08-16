//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 906/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk906(t198: f64, t530: f64, t1868: f64, t566: f64, t532: f64, t1907: f64, t4147: f64, t1317: f64, t1857: f64, t1320: f64, t1468: f64, t3833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5536 = t198 * t530;
    let t5537 = t566 * t1868;
    let t5541 = t198 * t532;
    let t5542 = t1907 * t4147;
    let t5545 = t1317 * t1857;
    let t5546 = 4.0_f64 * t5545;
    let t5547 = t1320 * t1857;
    let t5548 = 4.0_f64 * t5547;
    let t5549 = t3833 * t1468;
    (t5536, t5537, t5541, t5542, t5546, t5548, t5549)
}
