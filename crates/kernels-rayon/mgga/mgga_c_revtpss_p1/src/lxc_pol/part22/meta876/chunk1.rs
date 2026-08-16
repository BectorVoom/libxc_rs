//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3042/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3042(t4522: f64, t874: f64, t9288: f64, t1573: f64, t40317: f64, t14587: f64, t2782: f64, t39608: f64, t10069: f64, t14496: f64, t14524: f64, t39575: f64) -> (f64, f64, f64, f64, f64) {
    let t51445 = t874 * t4522 * t9288;
    let t51452 = t40317 * t1573;
    let t51460 = t2782 * t39608 * t14587;
    let t51470 = t10069 * t14496;
    let t51483 = t39575 * t14524;
    (t51445, t51452, t51460, t51470, t51483)
}
