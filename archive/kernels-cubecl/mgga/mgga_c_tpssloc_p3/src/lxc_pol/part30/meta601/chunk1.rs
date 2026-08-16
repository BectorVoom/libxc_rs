//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1990/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1990<F: Float>(t2627: F, t6624: F, t131: F, t2587: F, t81142: F, t1905: F, t9537: F, t81151: F, t23172: F, t133: F, t1891: F, t6601: F, t80953: F) -> (F, F, F, F, F, F) {
    let t81679 = t2627 * t6624;
    let t81686 = t81142 * t2587 * t131;
    let t81688 = t81686 * t9537 * t1905;
    let t81689 = F::cast_from(0.13707783890401886971e-2_f64) * t81688;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    let t81717 = F::cast_from(0.98696044010893586188e-1_f64) * t81716;
    let t81735 = t80953 * t1891 * t133 * t6601;
    (t81679, t81686, t81689, t81715, t81717, t81735)
}
