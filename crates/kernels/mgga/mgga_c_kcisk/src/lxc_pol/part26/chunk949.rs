//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 949/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk949<F: Float>(t25862: F, t321: F, t12884: F, t7802: F, t1210: F, t12888: F, t3696: F, t7819: F, t1543: F, t13064: F, t5794: F, t19580: F, t5720: F, t19476: F, t5753: F, t1190: F, t7754: F) -> (F, F, F, F, F, F, F) {
    let t25864 = 0.62182e-1 * t25862 * t321;
    let t25865 = t12884 * t7802;
    let t25866 = t12888 * t1210;
    let t25867 = t25865 * t25866;
    let t25870 = t3696 * t7819;
    let t25871 = t25870 * t1543;
    let t25874 = t13064 * t7802;
    let t25875 = t25874 * t5794;
    let t25879 = 4.0 * t19580 * t5720;
    let t25881 = 0.32163648644302209644e2 * t19476 * t5753;
    let t25882 = t7754 * t1190;
    (t25864, t25867, t25871, t25875, t25879, t25881, t25882)
}
