//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 710/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk710(t4580: f64, t70: f64, t1290: f64, t1306: f64, t2009: f64, t4573: f64, t4579: f64, t48: f64, t455: f64, t53: f64, t2016: f64, t60: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4581 = t4580 * t70;
    let t4584 = t1290 * t1306;
    let t4589 = t2009 * t4573;
    let t4592 = t48 * t4579;
    let t4596 = 1.0_f64 / t53 / t455;
    let t4597 = sigma2 * t4596;
    let t4602 = t2016 * t4573;
    let t4605 = t60 * t4579;
    (t4581, t4584, t4589, t4592, t4597, t4602, t4605)
}
