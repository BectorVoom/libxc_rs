//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 812/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk812(t1614: f64, t945: f64, t1622: f64, t953: f64, t2848: f64, t2906: f64, t2950: f64, t2957: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t4599: f64, t4607: f64, t4615: f64, t4617: f64, t4620: f64, t4623: f64, t4626: f64, t4629: f64) -> (f64, f64, f64) {
    let t4647 = t1614 * t945;
    let t4652 = t1622 * t953;
    let t4669 = -0.17648625e1_f64 * t4599 + 0.3529725e1_f64 * t4607 + t2950 + 0.17215833333333333333e0_f64 * t2848 + 0.17215833333333333333e0_f64 * t4571 - 0.34431666666666666667e0_f64 * t4576 + 0.103295e1_f64 * t4581 - 0.516475e0_f64 * t4585 + 0.31558125e0_f64 * t4615 + 0.6311625e0_f64 * t4617 + t2957 + 0.69463333333333333333e-1_f64 * t2906 + 0.69463333333333333333e-1_f64 * t4620 - 0.34731666666666666667e-1_f64 * t4623 + 0.20839e0_f64 * t4626 - 0.104195e0_f64 * t4629;
    (t4647, t4652, t4669)
}
