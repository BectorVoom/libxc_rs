//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1275/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1275<F: Float>(t12841: F, t32212: F, t3805: F, t9470: F, t10500: F, t2723: F, t109460: F, t9516: F, t1333: F, t32138: F, t32075: F, t32219: F, t32170: F, t9478: F, t4534: F, t9555: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t110066 = t12841 * t32212;
    let t110068 = t3805 * t9470;
    let t110077 = t10500 * t2723;
    let t110078 = 0.51588271604938271604e-3 * t110077;
    let t110079 = t9516 * t109460;
    let t110081 = t1333 * t32138;
    let t110092 = t1333 * t32075;
    let t110097 = t1333 * t32219;
    let t110099 = t1333 * t32170;
    let t110106 = t3805 * t9478;
    let t110120 = t9555 * t4534;
    (t110066, t110068, t110077, t110078, t110079, t110081, t110092, t110097, t110099, t110106, t110120)
}
