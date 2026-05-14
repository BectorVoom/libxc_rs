//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 981/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk981<F: Float>(t14208: F, t19881: F, t1340: F, t1411: F, t2152: F, t3786: F, t1450: F, t3785: F, t3494: F, t6001: F, t1415: F, t2231: F, t3777: F, t3776: F, t14213: F, t6006: F) -> (F, F, F, F, F, F, F) {
    let t19882 = t14208 * t19881;
    let t19883 = t1340 * t19882;
    let t19884 = t1411 * t19883;
    let t19886 = t2152 * t3786;
    let t19887 = t1450 * t19886;
    let t19888 = t3785 * t19887;
    let t19889 = t1411 * t19888;
    let t19891 = t3494 * t6001;
    let t19892 = t1415 * t19891;
    let t19893 = t1411 * t19892;
    let t19895 = t2231 * t3777;
    let t19896 = t3776 * t19895;
    let t19897 = t1415 * t19896;
    let t19898 = t1411 * t19897;
    let t19900 = t14213 * t6006;
    (t19884, t19886, t19889, t19893, t19895, t19898, t19900)
}
