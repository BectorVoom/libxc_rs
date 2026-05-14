//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1202/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1202<F: Float>(t23366: F, t179: F, t2405: F, t404: F, t7945: F, t1238: F, t6382: F, t6391: F, t3201: F, t5939: F, t918: F, t19193: F, t19196: F, t19206: F, t22260: F, t6369: F, t6395: F, t8319: F, t932: F) -> (F,) {
    let t23367 = 0.28582678745379824648e-3 * t23366;
    let t23375 = t404 * t179 * t2405 * t7945;
    let t23381 = t1238 * t6382;
    let t23382 = 0.15244095330869239812e-2 * t23381;
    let t23383 = t1238 * t6391;
    let t23388 = t918 * t5939 * t3201;
    let t23389 = 0.14291339372689912324e-3 * t23388;
    let t23390 = t23367 + 0.57165357490759649295e-3 * t19193 - 0.85748036236139473944e-3 * t19196 - 0.20579528696673473746e-1 * t8319 * t6369 - 0.34299214494455789578e-2 * t19206 - 0.85748036236139473944e-3 * t23375 - 0.42874018118069736972e-3 * t404 * t179 * t932 * t22260 - t23382 + 0.45732285992607719436e-2 * t23383 + 0.22866142996303859718e-2 * t1238 * t6395 - t23389;
    (t23390,)
}
