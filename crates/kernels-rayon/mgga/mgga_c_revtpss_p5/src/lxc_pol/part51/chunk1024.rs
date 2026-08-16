//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1024/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1024(t119808: f64, t31805: f64, t31801: f64, t2470: f64, t31800: f64, t31806: f64, t2670: f64, t31827: f64, t31809: f64, t31845: f64, t11007: f64, t3140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119809 = t31805 * t119808;
    let t119810 = t119809 * t31801;
    let t119813 = t31800 * t2470;
    let t119815 = 0.33852964522850660984e-1_f64 * t31806 * t119813;
    let t119816 = t31827 * t2670;
    let t119817 = 0.19833242244827205771e-3_f64 * t119816;
    let t119818 = t31809 * t31845;
    let t119821 = t3140 * t11007;
    (t119810, t119813, t119815, t119817, t119818, t119821)
}
