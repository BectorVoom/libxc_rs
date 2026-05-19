//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1321/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1321<F: Float>(t19191: F, t2380: F, t3224: F, t179: F, t2405: F, t404: F, t7945: F, t1238: F, t6382: F, t6391: F, t3201: F, t5939: F, t918: F) -> (F, F, F, F, F) {
    let t23366 = t2380 * t19191 * t3224;
    let t23367 = F::cast_from(0.28582678745379824648e-3_f64) * t23366;
    let t23375 = t404 * t179 * t2405 * t7945;
    let t23381 = t1238 * t6382;
    let t23382 = F::cast_from(0.15244095330869239812e-2_f64) * t23381;
    let t23383 = t1238 * t6391;
    let t23388 = t918 * t5939 * t3201;
    (t23367, t23375, t23382, t23383, t23388)
}
