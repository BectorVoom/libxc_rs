//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1467/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1467<F: Float>(t10623: F, t961: F, t2940: F, t2948: F, t2928: F, t941: F) -> (F, F, F) {
    let t10625 = F::cast_from(0.17544670867903938621e1_f64) * t10623 * t961;
    let t10627 = F::cast_from(0.17544670867903938621e1_f64) * t2940 * t2948;
    let t10629 = F::cast_from(1.0_f64) / t2928 / t941;
    (t10625, t10627, t10629)
}
