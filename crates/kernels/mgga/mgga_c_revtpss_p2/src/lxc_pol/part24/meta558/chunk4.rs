//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1673/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1673<F: Float>(t41610: F, t51978: F, t77736: F, t88118: F, t88126: F, t88134: F, t88168: F, t88171: F, t88203: F, t88206: F, t88209: F, t88211: F, t88214: F, t88216: F) -> F {
    let t88396 = F::new(0.197176e1) * t88168 + F::cast_from(0.49293999999999999999e0_f64) * t88171 + t41610 + F::cast_from(0.13145066666666666666e1_f64) * t77736 + F::cast_from(0.12401580246913580247e1_f64) * t51978 - F::cast_from(0.19931111111111111111e1_f64) * t88118 + F::cast_from(0.71752000000000000001e1_f64) * t88126 - F::cast_from(0.79724444444444444444e0_f64) * t88134 + F::new(0.1898925e1) * t88203 - F::cast_from(0.3560484375e1_f64) * t88206 - F::cast_from(0.10954222222222222222e0_f64) * t88209 + F::cast_from(0.1151859375e0_f64) * t88211 + F::new(0.46074375e0) * t88214 - F::new(0.28483875e1) * t88216;
    t88396
}
