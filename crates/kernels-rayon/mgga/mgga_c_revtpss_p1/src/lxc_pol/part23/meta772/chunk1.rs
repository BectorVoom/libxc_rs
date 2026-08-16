//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2576/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2576(t3717: f64, t57659: f64, t12865: f64, t17400: f64, t1222: f64, t1781: f64, t2438: f64, t12854: f64, t21013: f64, t12808: f64, t3698: f64, t5047: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57660 = t3717 * t57659;
    let t57663 = t17400 * t12865;
    let t57687 = t1222 * t2438 * t1781;
    let t57707 = t12854 * t21013;
    let t57710 = t12808 * t21013;
    let t57726 = t1222 * t697 * t3698 * t5047;
    (t57660, t57663, t57687, t57707, t57710, t57726)
}
