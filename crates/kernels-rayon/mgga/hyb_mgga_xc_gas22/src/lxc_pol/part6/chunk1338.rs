//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1338/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1338(t10742: f64, t2275: f64, t10703: f64, t2314: f64, t10743: f64, t10759: f64, t10778: f64, t10781: f64, t10782: f64, t10795: f64, t10798: f64, t10803: f64, t10806: f64, t10810: f64, t10823: f64, t20975: f64, t20995: f64, t21007: f64, t21048: f64, t21089: f64, t21091: f64, t2251: f64, t2252: f64, t2267: f64, t2273: f64, t2290: f64, t2312: f64, t24819: f64, t24822: f64, t24848: f64, t3389: f64, t3404: f64, t3423: f64, t4153: f64, t6636: f64, t6678: f64, t6710: f64, t6716: f64, t828: f64, t847: f64, t8763: f64, t8766: f64, t8824: f64, t8900: f64) -> f64 {
    let t29132 = t10742 * t2275;
    let t29157 = t10703 * t2314;
    let t29177 = -4.0_f64 * t2251 * t10743 * t828 + 0.64327917994770140268e2_f64 * t2273 * t29132 * t828 + 0.64327917994770140268e2_f64 * t2273 * t3389 * t8900 + 0.2069040516770936012e4_f64 * t6710 * t10781 * t2267 + 0.19964560303604640732e6_f64 * t21089 * t4153 * t21091 * t2252 - 0.23392894490538584828e1_f64 * t8824 * t8763 - 0.2077903092681775651e3_f64 * t24822 * t8766 - 0.23392894490538584828e1_f64 * t6716 * t10798 + 0.34631718211362927518e2_f64 * t6636 * t10803 - 0.23392894490538584828e1_f64 * t2290 * t10759 * t847 + 0.34631718211362927518e2_f64 * t2312 * t29157 * t847 - 0.2077903092681775651e3_f64 * t21007 * t10795 + 0.69263436422725855036e2_f64 * t6636 * t10806 + 0.20508037716432813316e4_f64 * t20995 * t10810 - 0.38596750796862084162e3_f64 * t20975 * t10823 + 0.12865583598954028054e3_f64 * t6678 * t10778 + 0.4138081033541872024e4_f64 * t21048 * t10782 - 0.46785788981077169656e1_f64 * t24819 * t3404 + 0.69263436422725855034e2_f64 * t24848 * t3423;
    t29177
}
