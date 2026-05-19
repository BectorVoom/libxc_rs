//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1121/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1121<F: Float>(t23340: F, t1229: F, t17955: F, t918: F, t1238: F, t6428: F, t19191: F, t2380: F, t3224: F, t6382: F, t3201: F, t5939: F) -> (F, F, F, F, F, F) {
    let t23341 = F::cast_from(0.7622047665434619906e-3_f64) * t23340;
    let t23345 = t918 * t17955 * t1229;
    let t23355 = t1238 * t6428;
    let t23366 = t2380 * t19191 * t3224;
    let t23367 = F::cast_from(0.28582678745379824648e-3_f64) * t23366;
    let t23381 = t1238 * t6382;
    let t23382 = F::cast_from(0.15244095330869239812e-2_f64) * t23381;
    let t23388 = t918 * t5939 * t3201;
    (t23341, t23345, t23355, t23367, t23382, t23388)
}
