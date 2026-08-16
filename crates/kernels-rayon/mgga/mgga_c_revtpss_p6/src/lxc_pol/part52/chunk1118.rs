//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1118/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1118(t2670: f64, t31827: f64, t31809: f64, t31845: f64, t11007: f64, t3140: f64, t822: f64, t31830: f64, t1032: f64, t7063: f64, t233: f64, t240: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119816 = t31827 * t2670;
    let t119818 = t31809 * t31845;
    let t119821 = t3140 * t11007;
    let t119822 = t119821 * t822;
    let t119823 = t31830 * t119822;
    let t119833 = t7063 * t1032;
    let t119835 = t233 * t27 * t240;
    (t119816, t119818, t119821, t119822, t119823, t119833, t119835)
}
