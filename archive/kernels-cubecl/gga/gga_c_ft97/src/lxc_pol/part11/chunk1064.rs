//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1064/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1064<F: Float>(t41950: F, t190: F, t2371: F, t251: F, t36452: F, t37991: F, t2476: F, t91: F, t2514: F, t2475: F, t1934: F, t2601: F) -> (F, F, F, F) {
    let t42044 = F::cast_from(280.0_f64) / F::cast_from(243.0_f64) * t41950;
    let t42050 = F::cast_from(1.0_f64) / t251 / t37991 / t190 / t2371 / t36452 / F::cast_from(96.0_f64);
    let t42051 = t2476 * t2476;
    let t42053 = t91 * t42050 * t42051;
    let t42055 = t2514 * t2514;
    let t42057 = t91 * t2475 * t42055;
    let t42059 = t2601 * t1934;
    (t42044, t42053, t42057, t42059)
}
