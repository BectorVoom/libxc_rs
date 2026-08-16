//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 960/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk960(t2554: f64, t2932: f64, t7064: f64, t5539: f64, t8769: f64, t9647: f64, t123: f64, t8773: f64, t2563: f64, t2558: f64, t8788: f64, t1843: f64, t8756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10691 = t2932 * t2554;
    let t10692 = t7064 * t10691;
    let t10693 = 0.32043859292259267849e-3_f64 * t10692;
    let t10694 = t5539 * t8769;
    let t10695 = t9647 * t10694;
    let t10696 = 0.64087718584518535698e-3_f64 * t10695;
    let t10697 = t8773 * t123;
    let t10698 = t10697 * t2563;
    let t10699 = t9647 * t10698;
    let t10700 = 0.96131577876777803547e-3_f64 * t10699;
    let t10701 = t8788 * t2558;
    let t10702 = t9647 * t10701;
    let t10703 = 0.32043859292259267849e-3_f64 * t10702;
    let t10704 = t1843 * t8756;
    (t10691, t10693, t10694, t10696, t10697, t10698, t10700, t10701, t10703, t10704)
}
