//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1105/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1105<F: Float>(t3446: F, t3447: F, t43936: F, t874: F, t122: F, t3434: F, t3437: F, t10619: F, t12567: F, t3262: F, t3264: F, t42444: F, t797: F, t8629: F, t3263: F, t11479: F, t11550: F) -> (F, F, F, F, F, F) {
    let t43939 = t3446 * t3447 * t43936 * t874;
    let t43943 = t3434 * t3437 * t43936 * t122;
    let t43946 = t12567 * t10619 / 4.0;
    let t43949 = 3.0 / 4.0 * t3262 * t42444 * t3264;
    let t43950 = t797 * t8629;
    let t43953 = 3.0 / 4.0 * t3262 * t3263 * t43950;
    let t43958 = 3.0 / 2.0 * t3262 * t11479 * t11550;
    (t43939, t43943, t43946, t43949, t43953, t43958)
}
