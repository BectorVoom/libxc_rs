//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2501/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2501(t10981: f64, t1579: f64, t22: f64, t868: f64, t15060: f64, t2435: f64, t14982: f64, t2465: f64, t2470: f64, t4480: f64, t9288: f64, t1569: f64, t2769: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t50178 = t10981 * t868 * t1579 * t22;
    let t50183 = t2435 * t15060;
    let t50184 = 0.21951497276451705329e-1_f64 * t50183;
    let t50186 = t2465 * t14982 * t2470;
    let t50187 = 0.39029762157531132076e-1_f64 * t50186;
    let t50205 = t2465 * t4480 * t9288;
    let t50208 = t786 * t1569 * t2769;
    (t50178, t50184, t50187, t50205, t50208)
}
