//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 988/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk988(t9147: f64, t9166: f64, t968: f64, t949: f64, t9112: f64, t9115: f64, t6969: f64, t6972: f64, t9119: f64, t9123: f64, t9127: f64, t9136: f64, t9138: f64, t9140: f64, t9143: f64, t9145: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9167 = t9147 + t9166;
    let t9168 = t9167 * t968;
    let t9170 = 1.0_f64 * t949 * t9168;
    let t9171 = 0.33114e0_f64 * t9112;
    let t9172 = 0.33114e0_f64 * t9115;
    let t9183 = -t9171 - t9172 + 0.248355e0_f64 * t9119 + 0.49671e0_f64 * t9123 + 0.248355e0_f64 * t9127 + 0.80513333333333333334e0_f64 * t6969 - 0.301925e0_f64 * t6972 + 0.258925e1_f64 * t9136 + 0.16504875e0_f64 * t9138 - 0.1294625e1_f64 * t9140 + 0.16504875e0_f64 * t9143 + 0.82524375e-1_f64 * t9145;
    (t9167, t9168, t9170, t9171, t9172, t9183)
}
