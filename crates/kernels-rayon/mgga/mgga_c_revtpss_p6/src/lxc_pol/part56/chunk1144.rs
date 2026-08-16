//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1144/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1144(t126004: f64, t126433: f64, t119675: f64, t119737: f64, t125997: f64, t126411: f64, t1544: f64, t1583: f64, t18875: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t27375: f64, t27384: f64, t31859: f64, t31863: f64, t33727: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t775: f64, t8490: f64, t890: f64, t892: f64) -> (f64, f64) {
    let t126434 = t126004 + t126433;
    let t127143 = t126411 * t198 * t207 * t892 + 2.0_f64 * t119675 * t1940 * t27384 - t119737 * t1583 * t1940 - t125997 * t1940 * t890 + 3.0_f64 * t1544 * t2403 * t31859 - 3.0_f64 * t18875 * t2403 * t31863 - t1940 * t31863 * t4537 - 3.0_f64 * t2403 * t27375 * t31863 + 3.0_f64 * t2403 * t33727 * t775 + 3.0_f64 * t2403 * t4343 * t8490 + 6.0_f64 * t4433 * t4541 * t8490;
    (t126434, t127143)
}
