//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2934/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2934(t41441: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64, t77858: f64) -> f64 {
    let t77974 = 0.34431666666666666667e0_f64 * t77559 - 0.103295e1_f64 * t77561 + 0.68863333333333333334e1_f64 * t77566 - 0.17215833333333333334e1_f64 * t77570 - 0.15302962962962962963e1_f64 * t77575 + 0.30872592592592592592e0_f64 * t41441 - 0.68863333333333333332e0_f64 * t63464 + 0.103295e1_f64 * t77581 - 0.34431666666666666667e0_f64 * t77586 - 0.123954e2_f64 * t77590 + 0.61977e1_f64 * t77594 + 0.69463333333333333333e-1_f64 * t77858;
    t77974
}
