//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 929/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk929(t27553: f64, t27592: f64, t27650: f64, t27706: f64, t3336: f64, t7840: f64, t1100: f64, t1699: f64, t1544: f64, t1583: f64, t18875: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2403: f64, t25440: f64, t25445: f64, t27363: f64, t27368: f64, t27375: f64, t27384: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t7087: f64, t7091: f64, t775: f64, t7783: f64, t890: f64, t892: f64) -> (f64, f64, f64, f64) {
    let t27708 = t27553 + t27592 + t27650 + t27706;
    let t27712 = t7840 * t3336;
    let t27717 = t1699 * t1100;
    let t27754 = t198 * t207 * t27363 * t892 + 3.0_f64 * t1544 * t2403 * t7087 - t1583 * t1940 * t25440 - 3.0_f64 * t18875 * t2403 * t7091 + 2.0_f64 * t1940 * t25445 * t27384 - t1940 * t27368 * t890 - t1940 * t4537 * t7091 + 3.0_f64 * t1963 * t2403 * t4343 + 6.0_f64 * t1963 * t4433 * t4541 - 3.0_f64 * t2403 * t27375 * t7091 + 3.0_f64 * t2403 * t775 * t7783;
    (t27708, t27712, t27717, t27754)
}
