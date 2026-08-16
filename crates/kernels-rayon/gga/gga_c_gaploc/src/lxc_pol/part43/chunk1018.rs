//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1018/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1018(t41697: f64, t41699: f64, t41700: f64, t41706: f64, t41712: f64, t41713: f64, t41714: f64, t41715: f64, t41716: f64, t41717: f64, t41718: f64, t41719: f64, t41721: f64, t47860: f64, t47864: f64, t47866: f64, t47869: f64, t47879: f64, t47883: f64, t47885: f64) -> f64 {
    let t50841 = -t41697 + t41699 - 0.42900587942220512004e1_f64 * t47860 - t41700 - 0.12423108009070322895e3_f64 * t47864 - 0.59584149919750711116e-1_f64 * t47866 - t41706 - t41712 + 0.19171462976960374838e1_f64 * t47869 + t41713 - t41714 - t41715 + t41716 + t41717 + t41718 + t41719 + 0.38342925953920749676e0_f64 * t47879 - 0.85206502119823888169e-1_f64 * t47883 + 0.38342925953920749676e0_f64 * t47885 + t41721;
    t50841
}
