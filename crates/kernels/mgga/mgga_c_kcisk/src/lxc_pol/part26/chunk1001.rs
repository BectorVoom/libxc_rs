//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1001/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1001<F: Float>(t1440: F, t8010: F, t3776: F, t1415: F, t1411: F, t14208: F, t26404: F, t1340: F, t19861: F, t2177: F, t1339: F, t1163: F, t8251: F, t3484: F, t3482: F, t3764: F, t8072: F) -> (F, F, F, F, F, F, F) {
    let t26889 = t8010 * t1440;
    let t26890 = t3776 * t26889;
    let t26891 = t1415 * t26890;
    let t26892 = t1411 * t26891;
    let t26894 = t14208 * t26404;
    let t26895 = t1340 * t26894;
    let t26896 = t1411 * t26895;
    let t26898 = t19861 * t2177;
    let t26899 = t1339 * t26898;
    let t26901 = t8251 * t1163;
    let t26902 = t3484 * t26901;
    let t26903 = t3482 * t26902;
    let t26905 = t3764 * t8072;
    (t26889, t26892, t26896, t26899, t26901, t26903, t26905)
}
