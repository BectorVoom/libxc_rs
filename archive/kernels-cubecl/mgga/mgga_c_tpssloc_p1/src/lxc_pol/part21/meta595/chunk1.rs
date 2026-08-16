//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2346/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2346<F: Float>(t556: F, t243: F, t3008: F, t343: F, t3034: F, t371: F) -> (F, F, F, F) {
    let t22842 = t556 * t556;
    let t22843 = F::cast_from(1.0_f64) / t22842;
    let t23075 = t243 * t243;
    let t23076 = F::cast_from(1.0_f64) / t23075;
    let t23494 = t3008 * t343;
    let t23508 = F::cast_from(1.0_f64) / t3034 / t371;
    (t22843, t23076, t23494, t23508)
}
