//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3086/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3086(t1134: f64, t24317: f64, t43821: f64, t20356: f64, t5079: f64, t24312: f64, t3390: f64, t16857: f64, t6449: f64, t20337: f64, t5071: f64, t43946: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81509 = t43821 * t24317 * t1134;
    let t81511 = t20356 * t5079;
    let t81513 = t3390 * t24312;
    let t81514 = t81513 * t1134;
    let t81516 = t16857 * t6449;
    let t81518 = t5071 * t20337;
    let t81521 = t43946 * t24317 * t1134;
    (t81509, t81511, t81514, t81516, t81518, t81521)
}
