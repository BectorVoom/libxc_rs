//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1207/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1207<F: Float>(t43: F, t50: F, t48542: F, t48544: F, t48546: F, t48548: F, t48550: F, t48552: F, t48554: F, t48556: F, t48558: F, t48560: F, zeta_threshold: F) -> F {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t48976 = piecewise3::<F>(t44, F::new(0.0), -F::new(56.0) / F::new(81.0) * t48542 + F::new(16.0) / F::new(9.0) * t48544 - F::new(2.0) / F::new(3.0) * t48546 - F::new(8.0) / F::new(9.0) * t48548 + F::new(2.0) / F::new(3.0) * t48550);
    let t48983 = piecewise3::<F>(t51, F::new(0.0), -F::new(56.0) / F::new(81.0) * t48552 + F::new(16.0) / F::new(9.0) * t48554 - F::new(2.0) / F::new(3.0) * t48556 - F::new(8.0) / F::new(9.0) * t48558 + F::new(2.0) / F::new(3.0) * t48560);
    let t48985 = t48976 / F::new(2.0) + t48983 / F::new(2.0);
    t48985
}
