//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 910/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk910<F: Float>(t12939: F, t1407: F, t2754: F, t587: F, t9438: F, t9439: F, t40007: F, t40021: F, t40023: F, t2877: F, t40394: F, t2299: F, t3338: F) -> (F, F, F, F, F, F, F) {
    let t41705 = t1407 * t12939;
    let t41706 = F::cast_from(0.15976219147466979032e-1_f64) * t41705;
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41712 = F::cast_from(0.15976219147466979032e-1_f64) * t41711;
    let t41713 = F::cast_from(0.29792074959875355558e-1_f64) * t40007;
    let t41718 = F::cast_from(0.29792074959875355558e-1_f64) * t40021;
    let t41719 = F::cast_from(0.29792074959875355558e-1_f64) * t40023;
    let t41721 = F::cast_from(0.35750489951850426669e0_f64) * t40394 * t2877;
    let t41722 = t2299 * t3338;
    (t41706, t41712, t41713, t41718, t41719, t41721, t41722)
}
