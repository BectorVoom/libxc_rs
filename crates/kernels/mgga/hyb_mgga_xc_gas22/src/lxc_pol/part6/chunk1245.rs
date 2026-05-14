//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1245/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1245<F: Float>(t10742: F, t2275: F, t10703: F, t2314: F, t10743: F, t10759: F, t10778: F, t10781: F, t10782: F, t10795: F, t10798: F, t10803: F, t10806: F, t10810: F, t10823: F, t20975: F, t20995: F, t21007: F, t21048: F, t21089: F, t21091: F, t2251: F, t2252: F, t2267: F, t2273: F, t2290: F, t2312: F, t24819: F, t24822: F, t24848: F, t3389: F, t3404: F, t3423: F, t4153: F, t6636: F, t6678: F, t6710: F, t6716: F, t828: F, t847: F, t8763: F, t8766: F, t8824: F, t8900: F) -> (F,) {
    let t29132 = t10742 * t2275;
    let t29157 = t10703 * t2314;
    let t29177 = -4.0 * t2251 * t10743 * t828 + 0.64327917994770140268e2 * t2273 * t29132 * t828 + 0.64327917994770140268e2 * t2273 * t3389 * t8900 + 0.2069040516770936012e4 * t6710 * t10781 * t2267 + 0.19964560303604640732e6 * t21089 * t4153 * t21091 * t2252 - 0.23392894490538584828e1 * t8824 * t8763 - 0.2077903092681775651e3 * t24822 * t8766 - 0.23392894490538584828e1 * t6716 * t10798 + 0.34631718211362927518e2 * t6636 * t10803 - 0.23392894490538584828e1 * t2290 * t10759 * t847 + 0.34631718211362927518e2 * t2312 * t29157 * t847 - 0.2077903092681775651e3 * t21007 * t10795 + 0.69263436422725855036e2 * t6636 * t10806 + 0.20508037716432813316e4 * t20995 * t10810 - 0.38596750796862084162e3 * t20975 * t10823 + 0.12865583598954028054e3 * t6678 * t10778 + 0.4138081033541872024e4 * t21048 * t10782 - 0.46785788981077169656e1 * t24819 * t3404 + 0.69263436422725855034e2 * t24848 * t3423;
    (t29177,)
}
