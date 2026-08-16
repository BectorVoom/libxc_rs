//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 578/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk578<F: Float>(t15014: F, t82: F, t72: F, t302: F, t3285: F, t275: F, t3286: F, t2339: F, t3056: F, t3057: F, t2323: F, t2338: F, t668: F) -> (F, F, F, F, F, F, F, F) {
    let t15015 = t82 * t15014;
    let t15016 = t72 * t15015;
    let t15017 = t302 * t3285;
    let t15018 = t72 * t15017;
    let t15020 = t275 * t3286;
    let t15030 = t3056 * t3057 * t2339;
    let t15033 = t3056 * t3057 * t2323;
    let t15035 = t2338 * t668;
    (t15015, t15016, t15017, t15018, t15020, t15030, t15033, t15035)
}
