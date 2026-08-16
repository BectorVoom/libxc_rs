//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1231/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1231<F: Float>(t1136: F, t4823: F, t3238: F, t3363: F, t4721: F, t4726: F, t4731: F, t4735: F) -> (F, F) {
    let t4824 = t4823 * t1136;
    let t4832 = t3363 - F::cast_from(0.30902777777777777778e-2_f64) * t3238 - F::cast_from(0.30902777777777777778e-2_f64) * t4721 - F::cast_from(0.61805555555555555555e-2_f64) * t4726 + F::cast_from(0.18541666666666666667e-1_f64) * t4731 + F::cast_from(0.92708333333333333333e-2_f64) * t4735;
    (t4824, t4832)
}
