//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2675/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2675<F: Float>(t2782: F, t4086: F, t49205: F, t543: F, t40270: F, t5737: F, t14127: F, t14193: F, t22016: F, t4056: F, t49177: F, t49178: F, t49187: F, t49190: F, t49199: F, t49200: F, t49203: F, t5735: F, t5745: F, t9840: F) -> F {
    let t49208 = t2782 * t4086 * t49205 * t543;
    let t49210 = t40270 * t5737;
    let t49212 = -t49177 + F::cast_from(0.17073386770573548589e-1_f64) * t49178 + t49187 - t49190 - F::cast_from(0.11853808529283920877e2_f64) * t14193 * t5735 * t22016 * t4056 + F::cast_from(0.39512695097613069591e1_f64) * t5745 * t14127 * t9840 + t49199 - F::cast_from(0.29272321618148349057e-1_f64) * t49200 - F::cast_from(0.30356481678079769392e-1_f64) * t49203 + F::cast_from(0.32927245914677557992e-1_f64) * t49208 - F::cast_from(0.2601984143835408805e-2_f64) * t49210;
    t49212
}
