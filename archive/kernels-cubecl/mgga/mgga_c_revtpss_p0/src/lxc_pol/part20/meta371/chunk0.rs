//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1348/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1348<F: Float>(t40196: F, t760: F, t14330: F, t189: F, t2251: F, t2258: F, t10587: F, t2626: F, t2523: F, t9425: F, t2389: F, t37: F) -> (F, F, F, F, F) {
    let t40198 = F::cast_from(0.35089341735807877242e1_f64) * t760 * t40196;
    let t40202 = F::cast_from(144.0_f64) * t14330 * t189 * t2251 * t2258;
    let t40203 = t10587 * t2626;
    let t40204 = F::cast_from(0.70178683471615754484e1_f64) * t40203;
    let t40205 = t2523 * t9425;
    let t40206 = F::cast_from(0.14035736694323150897e2_f64) * t40205;
    let t40207 = t37 * t2389;
    (t40198, t40202, t40204, t40206, t40207)
}
