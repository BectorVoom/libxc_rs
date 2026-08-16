//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1372/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1372(t23157: f64, t3961: f64, t102464: f64, t102467: f64, t21455: f64, t2237: f64, t28461: f64, t29300: f64, t29344: f64, t6176: f64, t7895: f64, t7901: f64, t7914: f64, t8148: f64, t8159: f64, t94589: f64, t98566: f64, t98568: f64, t98570: f64) -> f64 {
    let t103564 = t3961 * t23157;
    let t103572 = t94589 + 0.13901041666666666667e-2_f64 * t28461 * t8159 - 0.12356481481481481482e-2_f64 * t98566 - 0.12356481481481481482e-2_f64 * t98568 - 0.16489724537037037038e-3_f64 * t98570 - 0.24872916666666666666e-2_f64 * t102464 + 0.69505208333333333333e-3_f64 * t7895 * t29344 + 0.69505208333333333333e-3_f64 * t2237 * t6176 * t7914 * t21455 + 0.90693484953703703702e-3_f64 * t103564 * t7901 + 0.13901041666666666667e-2_f64 * t28461 * t8148 + 0.69505208333333333333e-3_f64 * t7895 * t29300 + 0.49745833333333333332e-2_f64 * t102467;
    t103572
}
