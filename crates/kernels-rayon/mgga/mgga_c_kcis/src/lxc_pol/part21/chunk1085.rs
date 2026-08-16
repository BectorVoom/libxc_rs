//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1085/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1085(t26806: f64, t7704: f64, t2894: f64, t3317: f64, t7726: f64, t303: f64, t26717: f64, t7690: f64, t2173: f64, t26779: f64, t26781: f64, t26784: f64, t26787: f64, t26793: f64, t26798: f64, t26801: f64, t26804: f64, t7693: f64, t7696: f64, t7703: f64, t7711: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26807 = t7704 * t26806;
    let t26808 = t2894 * t26807;
    let t26811 = t7726 * t3317;
    let t26812 = t303 * t26811;
    let t26814 = t7690 * t26717;
    let t26822 = 0.13265555555555555555e-1_f64 * t26779 - 0.185671721767578125e-4_f64 * t26781 * t26784 + 0.22109259259259259258e-2_f64 * t26787 - 0.13901041666666666667e-2_f64 * t2173 * t26784 - 0.13901041666666666667e-2_f64 * t2173 * t26793 - 0.49745833333333333332e-2_f64 * t26798 - 0.33163888888888888888e-2_f64 * t26801 + 0.22109259259259259258e-2_f64 * t26804 + 0.46336805555555555556e-3_f64 * t7703 * t26808 - 0.24872916666666666666e-2_f64 * t26812 + 0.61836467013888888889e-4_f64 * t26814 - 0.2782641015625e-3_f64 * t7690 * t26784 - 0.37069444444444444444e-2_f64 * t7696 * t7711 - 0.37069444444444444444e-2_f64 * t7696 * t7693;
    (t26807, t26808, t26811, t26812, t26814, t26822)
}
