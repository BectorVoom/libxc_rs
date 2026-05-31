//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1127/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1127<F: Float>(t19442: F, t7: F, t16089: F, t4811: F, t983: F, t1429: F, t1435: F, t444: F, t2503: F, t500: F, t23: F, t2499: F, t4819: F) -> (F, F, F, F, F, F) {
    let t19444 = F::cast_from(20.0_f64) * t7 * t19442;
    let t19446 = t16089 * t983 * t4811;
    let t19450 = t1435 * t1429 * t444;
    let t19453 = t2503 * t500;
    let t19455 = F::cast_from(20.0_f64) * t23 * t19453;
    let t19458 = t2499 * t4819;
    (t19444, t19446, t19450, t19453, t19455, t19458)
}
