//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 994/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk994<F: Float>(t1899: F, t23052: F, t1800: F, t1869: F, t17065: F, t2528: F, t22591: F, t682: F, t1814: F, t22632: F, t1806: F, t8537: F, t1850: F, t8491: F, t696: F, t8497: F) -> (F, F, F, F, F, F, F, F) {
    let t23053 = t1899 * t23052;
    let t23054 = t1800 * t23053;
    let t23055 = t1869 * t23054;
    let t23057 = t17065 * t2528;
    let t23058 = t1869 * t23057;
    let t23062 = t682 * t22591;
    let t23065 = t1814 * t22632;
    let t23068 = t1806 * t8537;
    let t23070 = t1850 * t8491;
    let t23072 = t696 * t8497;
    (t23053, t23055, t23058, t23062, t23065, t23068, t23070, t23072)
}
