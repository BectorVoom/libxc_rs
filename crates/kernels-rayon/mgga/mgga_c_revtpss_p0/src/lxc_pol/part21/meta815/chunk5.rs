//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2990/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2990(t11875: f64, t11922: f64, t15605: f64, t11852: f64, t41270: f64, t15905: f64, t43384: f64, t15595: f64, t3091: f64, t43131: f64, t11675: f64, t15984: f64) -> (f64, f64, f64, f64, f64) {
    let t54533 = t11875 * t11922 * t15605;
    let t54537 = t11852 * t41270;
    let t54542 = t43384 * t15905;
    let t54546 = t3091 * t43131 * t15595;
    let t54550 = t11675 * t15984;
    (t54533, t54537, t54542, t54546, t54550)
}
