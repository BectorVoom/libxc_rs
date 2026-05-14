//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 964/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk964<F: Float>(t2368: F, t394: F, t8429: F, t406: F, t3208: F, t926: F, t3206: F, t2382: F, t3265: F, t2381: F, t1249: F, t6483: F, t3188: F, t3185: F, t3224: F, t6475: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8451 = t2368 * t394;
    let t8452 = t8429 * t8451;
    let t8453 = t406 * t8452;
    let t8456 = t926 * t3208;
    let t8458 = 0.28582678745379824648e-3 * t3206 * t8456;
    let t8459 = t3265 * t2382;
    let t8460 = t2381 * t8459;
    let t8463 = t1249 * t6483;
    let t8464 = t2381 * t8463;
    let t8467 = t926 * t3188;
    let t8469 = 0.57165357490759649296e-3 * t3185 * t8467;
    let t8470 = t6475 * t3224;
    (t8451, t8452, t8453, t8458, t8459, t8460, t8463, t8464, t8469, t8470)
}
