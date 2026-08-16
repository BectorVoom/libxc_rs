//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 845/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk845(t10525: f64, t10526: f64, t41878: f64, t30829: f64, t31769: f64, t544: f64, t913: f64, t1424: f64, t2875: f64, t9060: f64, t10405: f64, t2478: f64, t6583: f64) -> (f64, f64, f64, f64) {
    let t41880 = t10525 * t10526 * t41878;
    let t41884 = t544 * t30829 * t913 * t31769;
    let t41885 = 0.3575048995185042667e0_f64 * t41884;
    let t41889 = 0.39722766613167140743e-1_f64 * t544 * t9060 * t2875 * t1424;
    let t41891 = t6583 * t10405 * t2478;
    (t41880, t41885, t41889, t41891)
}
