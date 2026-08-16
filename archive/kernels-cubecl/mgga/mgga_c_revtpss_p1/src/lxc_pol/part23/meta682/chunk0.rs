//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2423/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2423<F: Float>(t43813: F, t43816: F, t3475: F, t426: F, t3478: F, t1175: F, t12552: F, t43752: F, t439: F, t3519: F, t3522: F, t1156: F, t12428: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t45106 = F::cast_from(0.5356037037037037037e1_f64) * t43813;
    let t45107 = F::cast_from(0.16979925925925925926e1_f64) * t43816;
    let t45155 = t3475 * t3475;
    let t45157 = t426 / t45155;
    let t45158 = t3478 * t3478;
    let t45159 = F::cast_from(1.0_f64) / t45158;
    let t45174 = t1175 * t12552;
    let t45177 = t439 * t43752;
    let t45186 = t3519 * t3519;
    let t45187 = F::cast_from(1.0_f64) / t45186;
    let t45188 = t439 * t45187;
    let t45189 = t3522 * t3522;
    let t45190 = F::cast_from(1.0_f64) / t45189;
    let t45197 = t1156 * t12428;
    (t45106, t45107, t45157, t45159, t45174, t45177, t45187, t45188, t45190, t45197)
}
