//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1179/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1179<F: Float>(t41718: F, t41719: F, t41721: F, t41724: F, t41729: F, t41731: F, t47873: F, t47874: F, t47875: F, t47879: F, t47883: F, t47885: F) -> F {
    let t47889 = -t47873 + t47874 + t47875 + t41718 + t41719 + F::cast_from(0.19171462976960374838e0_f64) * t47879 - F::cast_from(0.42603251059911944084e-1_f64) * t47883 + F::cast_from(0.19171462976960374838e0_f64) * t47885 + t41721 - F::cast_from(0.35750489951850426669e0_f64) * t41724 - t41729 + F::cast_from(0.29792074959875355558e-1_f64) * t41731;
    t47889
}
