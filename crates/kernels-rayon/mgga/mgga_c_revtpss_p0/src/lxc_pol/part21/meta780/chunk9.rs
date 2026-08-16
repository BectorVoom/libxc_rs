//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2790/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2790(t10995: f64, t11049: f64, t14990: f64, t14986: f64, t2453: f64, t10506: f64, t2458: f64, t4470: f64, t10069: f64, t14482: f64, t15003: f64, t41020: f64) -> (f64, f64, f64, f64, f64) {
    let t51256 = t10995 * t14990 * t11049;
    let t51258 = t2453 * t14986;
    let t51259 = t51258 * t10506;
    let t51260 = 0.34697458558045176417e-2_f64 * t51259;
    let t51262 = t2453 * t4470 * t2458;
    let t51263 = 0.34697458558045176417e-2_f64 * t51262;
    let t51264 = t10069 * t14482;
    let t51268 = t41020 * t15003;
    (t51256, t51260, t51263, t51264, t51268)
}
