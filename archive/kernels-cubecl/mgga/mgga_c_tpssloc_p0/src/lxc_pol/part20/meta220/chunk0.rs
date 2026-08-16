//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1289/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1289<F: Float>(t334: F, t371: F, t533: F, t556: F, t1351: F, t562: F, t1388: F, t3701: F, t1184: F, t460: F, t1433: F, t71: F) -> (F, F, F, F, F, F) {
    let t6793 = t371 * t334;
    let t6924 = F::cast_from(1.0_f64) / t556 / t533;
    let t6977 = t562 * t1351;
    let t6999 = t3701 * t1388;
    let t7319 = t1184 * t460;
    let t7445 = t71 * t1433;
    (t6793, t6924, t6977, t6999, t7319, t7445)
}
