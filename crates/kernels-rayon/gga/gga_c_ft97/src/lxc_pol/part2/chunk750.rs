//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 750/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk750(t11076: f64, t1808: f64, t3119: f64, t91: f64, t1767: f64, t8345: f64, t965: f64, t1766: f64, t3157: f64, t473: f64, t11416: f64, t11395: f64, t11399: f64, t11404: f64, t11408: f64, t11413: f64, t8455: f64) -> (f64, f64, f64, f64) {
    let t11781 = 4.0_f64 / 27.0_f64 * t11076;
    let t11783 = t91 * t3119 * t1808;
    let t11787 = t91 * t8345 * t965 * t1767;
    let t11789 = t1766 * t3157;
    let t11791 = t91 * t11789 * t473;
    let t11798 = 4.0_f64 / 9.0_f64 * t11416;
    let t11799 = -t11781 - t8455 - t11783 / 12.0_f64 + t11787 / 8.0_f64 - t11791 / 6.0_f64 - t11395 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t11399 + 22.0_f64 / 27.0_f64 * t11404 + 2.0_f64 / 3.0_f64 * t11408 + 4.0_f64 / 3.0_f64 * t11413 - t11798;
    (t11783, t11787, t11791, t11799)
}
