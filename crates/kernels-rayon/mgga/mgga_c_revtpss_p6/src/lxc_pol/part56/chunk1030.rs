//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1030/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1030(t119821: f64, t822: f64, t31830: f64, t122: f64, t72: f64, t8471: f64, t2466: f64, t25377: f64, t676: f64, t7048: f64, t32474: f64, t1032: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t119822 = t119821 * t822;
    let t119823 = t31830 * t119822;
    let t119825 = t8471 * t72 * t122;
    let t119826 = t119825 * t2466;
    let t119827 = t119823 * t119826;
    let t119830 = t25377 * t676 * t7048;
    let t119831 = t32474 * t119830;
    let t119833 = t7063 * t1032;
    (t119822, t119823, t119825, t119826, t119827, t119830, t119831, t119833)
}
