//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1085/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1085<F: Float>(t26806: F, t7704: F, t2894: F, t3317: F, t7726: F, t303: F, t26717: F, t7690: F, t2173: F, t26779: F, t26781: F, t26784: F, t26787: F, t26793: F, t26798: F, t26801: F, t26804: F, t7693: F, t7696: F, t7703: F, t7711: F) -> (F, F, F, F, F, F) {
    let t26807 = t7704 * t26806;
    let t26808 = t2894 * t26807;
    let t26811 = t7726 * t3317;
    let t26812 = t303 * t26811;
    let t26814 = t7690 * t26717;
    let t26822 = F::new(0.13265555555555555555e-1) * t26779 - F::new(0.185671721767578125e-4) * t26781 * t26784 + F::new(0.22109259259259259258e-2) * t26787 - F::new(0.13901041666666666667e-2) * t2173 * t26784 - F::new(0.13901041666666666667e-2) * t2173 * t26793 - F::new(0.49745833333333333332e-2) * t26798 - F::new(0.33163888888888888888e-2) * t26801 + F::new(0.22109259259259259258e-2) * t26804 + F::new(0.46336805555555555556e-3) * t7703 * t26808 - F::new(0.24872916666666666666e-2) * t26812 + F::new(0.61836467013888888889e-4) * t26814 - F::new(0.2782641015625e-3) * t7690 * t26784 - F::new(0.37069444444444444444e-2) * t7696 * t7711 - F::new(0.37069444444444444444e-2) * t7696 * t7693;
    (t26807, t26808, t26811, t26812, t26814, t26822)
}
