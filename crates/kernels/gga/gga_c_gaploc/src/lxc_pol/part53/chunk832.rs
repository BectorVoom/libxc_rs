//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 832/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk832<F: Float>(t41705: F, t2754: F, t587: F, t9438: F, t9439: F, t40007: F, t40009: F, t40013: F, t40015: F, t40019: F, t40021: F, t40023: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41706 = F::new(0.15976219147466979032e-1) * t41705;
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41712 = F::new(0.15976219147466979032e-1) * t41711;
    let t41713 = F::new(0.29792074959875355558e-1) * t40007;
    let t41714 = F::new(0.63904876589867916127e-1) * t40009;
    let t41715 = F::new(0.63904876589867916127e-1) * t40013;
    let t41716 = F::new(0.63904876589867916127e-1) * t40015;
    let t41717 = F::new(0.63904876589867916127e-1) * t40019;
    let t41718 = F::new(0.29792074959875355558e-1) * t40021;
    let t41719 = F::new(0.29792074959875355558e-1) * t40023;
    (t41706, t41712, t41713, t41714, t41715, t41716, t41717, t41718, t41719)
}
