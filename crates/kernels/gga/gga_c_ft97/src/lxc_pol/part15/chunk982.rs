//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 982/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk982<F: Float>(t22116: F, t8959: F, t22100: F, t39942: F, t21130: F, t703: F, t801: F, t1109: F, t5295: F, t21249: F, t816: F, t5260: F, t817: F) -> (F, F, F, F, F, F) {
    let t83088 = t8959 * t22116;
    let t83103 = F::cast_from(0.22136921132726965153e-3_f64) * t39942 * t22100;
    let t83109 = t703 * t21130 * t801;
    let t83158 = t1109 * t5295;
    let t83210 = t816 * t21249;
    let t83232 = t817 * t5260;
    (t83088, t83103, t83109, t83158, t83210, t83232)
}
