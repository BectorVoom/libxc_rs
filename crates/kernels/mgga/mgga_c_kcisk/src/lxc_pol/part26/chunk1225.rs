//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1225/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1225<F: Float>(t109882: F, t382: F, t1310: F, t1588: F, t3951: F, t1405: F, t3783: F, t3805: F, t9470: F, t10500: F, t2723: F, t9478: F, t4534: F, t9555: F, t15093: F, t2744: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109883 = t109882 * t382;
    let t109963 = t1310 * t3951 * t1588;
    let t110016 = t1405 * t3783;
    let t110068 = t3805 * t9470;
    let t110077 = t10500 * t2723;
    let t110078 = 0.51588271604938271604e-3 * t110077;
    let t110106 = t3805 * t9478;
    let t110120 = t9555 * t4534;
    let t110136 = t2744 * t15093;
    (t109883, t109963, t110016, t110068, t110077, t110078, t110106, t110120, t110136)
}
