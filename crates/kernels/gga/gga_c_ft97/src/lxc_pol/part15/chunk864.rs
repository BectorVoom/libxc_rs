//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 864/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk864<F: Float>(t1095: F, t70402: F, t22107: F, t8959: F, t22111: F, t39922: F, t22116: F, t22100: F, t39942: F, t21130: F, t703: F, t801: F, t1109: F, t5295: F, t21249: F, t816: F) -> (F, F, F, F, F, F, F, F) {
    let t83049 = t70402 * t1095;
    let t83084 = t8959 * t22107;
    let t83086 = t39922 * t22111;
    let t83088 = t8959 * t22116;
    let t83103 = 0.22136921132726965153e-3 * t39942 * t22100;
    let t83109 = t703 * t21130 * t801;
    let t83158 = t1109 * t5295;
    let t83210 = t816 * t21249;
    (t83049, t83084, t83086, t83088, t83103, t83109, t83158, t83210)
}
