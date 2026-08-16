//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 880/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk880(t1794: f64, t2142: f64, t73: f64, t1203: f64, t5457: f64, t5458: f64, t1294: f64, t5215: f64, t7637: f64, t1828: f64, t7627: f64, t7652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29157 = t2142 * t1794;
    let t29158 = t29157 * t73;
    let t29159 = t5457 * t1203;
    let t29160 = t29158 * t29159;
    let t29163 = t29158 * t5458;
    let t29166 = t5457 * t1294;
    let t29167 = t29158 * t29166;
    let t29174 = t2142 * t5215;
    let t29175 = t7637 * t29174;
    let t29178 = t7627 * t1828;
    let t29179 = t7652 * t29178;
    (t29157, t29158, t29159, t29160, t29163, t29166, t29167, t29175, t29179)
}
