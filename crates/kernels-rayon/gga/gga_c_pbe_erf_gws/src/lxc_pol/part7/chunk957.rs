//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 957/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk957(t401: f64, t5253: f64, t1764: f64, t177: f64, t191: f64, t16613: f64, t16675: f64, t16684: f64, t16699: f64, t16701: f64, t16708: f64, t16718: f64, t16720: f64, t16726: f64, t16741: f64, t16747: f64, t1856: f64, t25: f64, t5264: f64, t606: f64) -> f64 {
    let t17745 = t401 * t5253;
    let t17758 = t191 / t177 / t1764;
    let t17765 = 0.28793333333333333333e0_f64 * t16701 - 0.28793333333333333333e0_f64 * t16708 - 0.23994444444444444446e0_f64 * t16720 + 0.95977777777777777777e-1_f64 * t16726 - 0.88888888888888888888e-2_f64 * t25 * t1856 * t16684 - 0.17777777777777777778e-1_f64 * t25 * t5264 * t16718 + 0.17777777777777777778e-1_f64 * t17745 - 0.24e0_f64 * t25 * t606 * t16613 + 0.53333333333333333332e-1_f64 * t25 * t606 * t16699 + 0.79999999999999999998e-1_f64 * t25 * t1856 * t16675 - 0.69135802469135802468e-2_f64 * t25 * t17758 * t16741 - 0.66666666666666666667e-2_f64 * t25 * t606 * t16747;
    t17765
}
