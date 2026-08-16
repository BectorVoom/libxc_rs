//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1114/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1114(t10926: f64, t10952: f64, t968: f64, t949: f64, t4273: f64, t7070: f64, t10898: f64, t10913: f64, t6967: f64, t6969: f64, t9008: f64, t9012: f64) -> (f64, f64, f64, f64, f64) {
    let t10953 = t10926 + t10952;
    let t10954 = t10953 * t968;
    let t10956 = 1.0_f64 * t949 * t10954;
    let t10958 = 0.16081979498692535067e2_f64 * t7070 * t4273;
    let t10963 = -t6967 + 0.12361111111111111111e-1_f64 * t6969 + 0.24722222222222222223e-1_f64 * t9008 - t9012 - 0.92708333333333333333e-2_f64 * t10898 + 0.278125e-1_f64 * t10913;
    (t10953, t10954, t10956, t10958, t10963)
}
