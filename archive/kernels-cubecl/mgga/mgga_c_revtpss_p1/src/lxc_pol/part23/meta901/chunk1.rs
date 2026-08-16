//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2868/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2868<F: Float>(t40271: F, t40294: F, t4514: F, t51507: F, t62777: F, t62809: F, t76127: F, t76136: F, t77171: F, t77177: F, t77183: F, t77191: F, t837: F) -> F {
    let t77193 = F::cast_from(0.16463622957338778997e-1_f64) * t77171 + F::cast_from(0.43902994552903410656e-1_f64) * t62777 - F::cast_from(0.26019841438354088051e-2_f64) * t40271 - F::cast_from(0.32927245914677557992e-1_f64) * t77177 - t40294 - F::cast_from(0.65854491829355115987e0_f64) * t4514 * t76127 * t837 - F::cast_from(0.29272321618148349057e-1_f64) * t62809 - F::cast_from(0.29272321618148349057e-1_f64) * t77183 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t76136 * t837 + F::cast_from(0.43902994552903410658e-1_f64) * t51507 + F::cast_from(0.32927245914677557992e-1_f64) * t77191;
    t77193
}
