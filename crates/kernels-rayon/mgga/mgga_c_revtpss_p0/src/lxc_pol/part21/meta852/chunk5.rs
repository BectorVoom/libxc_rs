//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3207/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3207(t12916: f64, t17709: f64, t17712: f64, t3766: f64, t5219: f64, t5330: f64, t17601: f64, t3718: f64, t12855: f64, t17579: f64, t12809: f64, t17483: f64) -> (f64, f64, f64, f64, f64) {
    let t59159 = t17709 * t12916 * t17712;
    let t59162 = t5219 * t3766 * t5330;
    let t59173 = t3718 * t12916 * t17601;
    let t59176 = t12855 * t12916 * t17579;
    let t59179 = t12809 * t12916 * t17483;
    (t59159, t59162, t59173, t59176, t59179)
}
