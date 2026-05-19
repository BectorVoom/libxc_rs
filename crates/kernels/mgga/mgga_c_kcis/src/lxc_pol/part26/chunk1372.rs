//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1372/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1372<F: Float>(t23157: F, t3961: F, t102464: F, t102467: F, t21455: F, t2237: F, t28461: F, t29300: F, t29344: F, t6176: F, t7895: F, t7901: F, t7914: F, t8148: F, t8159: F, t94589: F, t98566: F, t98568: F, t98570: F) -> F {
    let t103564 = t3961 * t23157;
    let t103572 = t94589 + F::cast_from(0.13901041666666666667e-2_f64) * t28461 * t8159 - F::cast_from(0.12356481481481481482e-2_f64) * t98566 - F::cast_from(0.12356481481481481482e-2_f64) * t98568 - F::cast_from(0.16489724537037037038e-3_f64) * t98570 - F::cast_from(0.24872916666666666666e-2_f64) * t102464 + F::cast_from(0.69505208333333333333e-3_f64) * t7895 * t29344 + F::cast_from(0.69505208333333333333e-3_f64) * t2237 * t6176 * t7914 * t21455 + F::cast_from(0.90693484953703703702e-3_f64) * t103564 * t7901 + F::cast_from(0.13901041666666666667e-2_f64) * t28461 * t8148 + F::cast_from(0.69505208333333333333e-3_f64) * t7895 * t29300 + F::cast_from(0.49745833333333333332e-2_f64) * t102467;
    t103572
}
