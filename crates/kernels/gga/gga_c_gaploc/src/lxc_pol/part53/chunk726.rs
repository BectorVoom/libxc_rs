//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 726/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk726<F: Float>(t41698: F, t39968: F, t12939: F, t1407: F, t2754: F, t587: F, t9438: F, t9439: F, t40007: F, t40009: F, t40013: F, t40015: F, t40019: F, t40021: F, t40023: F, t2877: F, t40394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41699 = 0.1022478025437886658e1 * t41698;
    let t41700 = 0.19171462976960374838e1 * t39968;
    let t41705 = t1407 * t12939;
    let t41706 = 0.15976219147466979032e-1 * t41705;
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41712 = 0.15976219147466979032e-1 * t41711;
    let t41713 = 0.29792074959875355558e-1 * t40007;
    let t41714 = 0.63904876589867916127e-1 * t40009;
    let t41715 = 0.63904876589867916127e-1 * t40013;
    let t41716 = 0.63904876589867916127e-1 * t40015;
    let t41717 = 0.63904876589867916127e-1 * t40019;
    let t41718 = 0.29792074959875355558e-1 * t40021;
    let t41719 = 0.29792074959875355558e-1 * t40023;
    let t41721 = 0.35750489951850426669e0 * t40394 * t2877;
    (t41699, t41700, t41706, t41712, t41713, t41714, t41715, t41716, t41717, t41718, t41719, t41721)
}
