//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3223/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3223(t12050: f64, t17710: f64, t17191: f64, t3555: f64, t1269: f64, t13147: f64, t460: f64, t1209: f64, t21455: f64, t5219: f64, t5477: f64, t17288: f64, t3754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59650 = t17710 * t12050;
    let t59657 = t3555 * t17191;
    let t59671 = t460 * t13147 * t1269;
    let t59674 = t1209 * t21455;
    let t59681 = t5219 * t5477;
    let t59686 = t17288 * t3754;
    (t59650, t59657, t59671, t59674, t59681, t59686)
}
