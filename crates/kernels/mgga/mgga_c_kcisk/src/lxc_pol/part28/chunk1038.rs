//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1038/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1038<F: Float>(t4811: F, t8941: F, t6708: F, t6719: F, t1869: F, t6974: F, t7070: F, t11213: F, t23733: F, t1800: F, t1636: F, t23033: F, t1799: F, t1060: F, t2441: F, t6763: F) -> (F, F, F, F, F, F, F) {
    let t23894 = t4811 * t8941;
    let t23897 = t6719 * t6708;
    let t23898 = t1869 * t23897;
    let t23901 = t6974 * t7070;
    let t23902 = t1869 * t23901;
    let t23904 = t11213 * t23733;
    let t23905 = t1800 * t23904;
    let t23906 = t1869 * t23905;
    let t23908 = t23033 * t1636;
    let t23909 = t1800 * t23908;
    let t23910 = t1799 * t23909;
    let t23912 = t2441 * t1060;
    let t23913 = t6763 * t23912;
    (t23894, t23898, t23902, t23906, t23910, t23912, t23913)
}
