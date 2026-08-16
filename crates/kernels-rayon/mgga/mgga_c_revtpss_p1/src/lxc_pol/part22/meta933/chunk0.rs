//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3163/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3163(t12916: f64, t17780: f64, t5331: f64, t1260: f64, t45385: f64, t12640: f64, t17728: f64, t489: f64, t12744: f64, t17350: f64, t3781: f64, t5219: f64, t5330: f64) -> (f64, f64, f64, f64, f64) {
    let t57336 = t5331 * t12916 * t17780;
    let t57344 = t45385 * t1260;
    let t57348 = t12640 * t489 * t17728;
    let t57378 = t12744 * t17350;
    let t57382 = t5219 * t3781 * t5330;
    (t57336, t57344, t57348, t57378, t57382)
}
