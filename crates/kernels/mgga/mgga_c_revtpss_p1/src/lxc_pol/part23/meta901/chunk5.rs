//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2872/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2872<F: Float>(t18677: F, t18714: F, t4424: F, t4514: F, t51635: F, t51637: F, t51646: F, t51657: F, t62983: F, t62987: F, t62992: F, t62999: F, t76169: F, t820: F, t837: F) -> F {
    let t77278 = F::cast_from(0.7805952431506226415e-1_f64) * t62983 + F::cast_from(0.11708928647259339623e0_f64) * t62987 + F::cast_from(0.58544643236296698114e-1_f64) * t62992 + F::cast_from(0.13878983423218070567e-1_f64) * t51635 + F::cast_from(0.19514881078765566038e-2_f64) * t51637 - F::cast_from(0.34697458558045176418e-2_f64) * t62999 - F::cast_from(0.13878983423218070567e-1_f64) * t51646 - F::cast_from(0.78059524315062264152e-1_f64) * t51657 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t18714 * t4424 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t18677 * t4424 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t76169 * t837;
    t77278
}
