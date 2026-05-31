//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2937/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2937<F: Float>(t41361: F, t51978: F, t52701: F, t63320: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77736: F, t77739: F) -> F {
    let t78035 = F::cast_from(0.35876e1_f64) * t77515 - F::cast_from(0.99655555555555555555e0_f64) * t77518 - F::cast_from(0.53814e1_f64) * t77521 + F::cast_from(0.32862666666666666666e0_f64) * t77736 - F::cast_from(0.147882e1_f64) * t77739 - t52701 + F::cast_from(0.93011851851851851854e0_f64) * t51978 + F::cast_from(0.16431333333333333333e0_f64) * t63320 + F::cast_from(0.31003950617283950618e0_f64) * t41361 - F::cast_from(0.59793333333333333333e0_f64) * t77527 - F::cast_from(0.59793333333333333333e0_f64) * t77531 + F::cast_from(0.71752e1_f64) * t77535;
    t78035
}
