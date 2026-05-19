//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1017/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1017<F: Float>(t41697: F, t41699: F, t41700: F, t41706: F, t41712: F, t41713: F, t41714: F, t41715: F, t41716: F, t41717: F, t41718: F, t41719: F, t41721: F, t47860: F, t47864: F, t47866: F, t47869: F, t47879: F, t47883: F, t47885: F) -> F {
    let t50841 = -t41697 + t41699 - F::cast_from(0.42900587942220512004e1_f64) * t47860 - t41700 - F::cast_from(0.12423108009070322895e3_f64) * t47864 - F::cast_from(0.59584149919750711116e-1_f64) * t47866 - t41706 - t41712 + F::cast_from(0.19171462976960374838e1_f64) * t47869 + t41713 - t41714 - t41715 + t41716 + t41717 + t41718 + t41719 + F::cast_from(0.38342925953920749676e0_f64) * t47879 - F::cast_from(0.85206502119823888169e-1_f64) * t47883 + F::cast_from(0.38342925953920749676e0_f64) * t47885 + t41721;
    t50841
}
