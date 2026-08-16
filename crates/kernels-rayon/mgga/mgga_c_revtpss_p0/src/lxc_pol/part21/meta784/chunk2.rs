//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2823/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2823(t2430: f64, t890: f64, t14397: f64, t1940: f64, t2403: f64, t2832: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t4556: f64, t50899: f64, t50900: f64, t50902: f64, t50905: f64, t50907: f64) -> f64 {
    let t51806 = t2430 * t890;
    let t51810 = -3.0_f64 * t14397 * t1940 * t2832 - 9.0_f64 * t2403 * t4556 * t51806 + t40076 - t40079 + t40194 + t40198 + t50899 - t50900 - t50902 + t50905 + t50907;
    t51810
}
