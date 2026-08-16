//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1393/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1393<F: Float>(t5914: F, t990: F, t17875: F, t381: F, t1049: F, t5848: F, t1065: F, t5943: F, t3174: F, t1625: F, t4552: F, t5919: F) -> (F, F, F, F, F, F) {
    let t18053 = t990 * t5914;
    let t18057 = t17875 * t381;
    let t18059 = t5848 * t1049;
    let t18061 = t5943 * t1065;
    let t18062 = t3174 * t18061;
    let t18065 = t4552 * t1625;
    let t18070 = t5919 * t1065;
    (t18053, t18057, t18059, t18062, t18065, t18070)
}
