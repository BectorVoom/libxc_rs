//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2405/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2405<F: Float>(t39264: F, t761: F, t2663: F, t9901: F, t2531: F, t9905: F, t39259: F, t2250: F, t2517: F, t707: F, t39358: F, t756: F) -> (F, F, F, F, F, F) {
    let t40679 = F::cast_from(0.61524113149298439947e4_f64) * t761 * t39264;
    let t40680 = t9901 * t2663;
    let t40682 = t2531 * t9905;
    let t40685 = F::cast_from(0.69263436422725855036e2_f64) * t761 * t39259;
    let t40687 = t707 * t2517 * t2250;
    let t40708 = F::cast_from(0.18989649058080861537e-2_f64) * t756 * t39358;
    (t40679, t40680, t40682, t40685, t40687, t40708)
}
