//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3130/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3130(t1222: f64, t1781: f64, t2438: f64, t12886: f64, t5391: f64, t3601: f64, t5245: f64, t12854: f64, t21013: f64, t12808: f64, t3698: f64, t5047: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57687 = t1222 * t2438 * t1781;
    let t57689 = t5391 * t12886;
    let t57696 = t5245 * t3601;
    let t57707 = t12854 * t21013;
    let t57710 = t12808 * t21013;
    let t57726 = t1222 * t697 * t3698 * t5047;
    (t57687, t57689, t57696, t57707, t57710, t57726)
}
