//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 988/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk988<F: Float>(t9147: F, t9166: F, t968: F, t949: F, t9112: F, t9115: F, t6969: F, t6972: F, t9119: F, t9123: F, t9127: F, t9136: F, t9138: F, t9140: F, t9143: F, t9145: F) -> (F, F, F, F, F, F) {
    let t9167 = t9147 + t9166;
    let t9168 = t9167 * t968;
    let t9170 = F::new(1.0) * t949 * t9168;
    let t9171 = F::new(0.33114e0) * t9112;
    let t9172 = F::new(0.33114e0) * t9115;
    let t9183 = -t9171 - t9172 + F::new(0.248355e0) * t9119 + F::new(0.49671e0) * t9123 + F::new(0.248355e0) * t9127 + F::new(0.80513333333333333334e0) * t6969 - F::new(0.301925e0) * t6972 + F::new(0.258925e1) * t9136 + F::new(0.16504875e0) * t9138 - F::new(0.1294625e1) * t9140 + F::new(0.16504875e0) * t9143 + F::new(0.82524375e-1) * t9145;
    (t9167, t9168, t9170, t9171, t9172, t9183)
}
