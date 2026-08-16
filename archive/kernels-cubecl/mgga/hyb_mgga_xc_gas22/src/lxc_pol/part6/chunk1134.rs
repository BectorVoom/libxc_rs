//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1134/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1134<F: Float>(t11186: F, t7276: F, t7278: F, t7310: F, t7312: F, t7314: F, t7484: F, t7487: F, t7490: F, t7493: F, t7496: F, t7498: F, t9338: F, t9372: F, t9375: F, t9379: F, t9381: F) -> F {
    let t11230 = t9338 + F::cast_from(12.0_f64) * t7276 + F::cast_from(32.0_f64) * t7278 + t7310 + t7312 - F::cast_from(16.0_f64) * t9372 + F::cast_from(2.0_f64) * t9375 - t9379 - t9381 + t7314 + t7484 - t11186 - t7487 - t7490 + t7493 + t7496 + F::cast_from(0.10843581300301739842e-1_f64) * t7498;
    t11230
}
