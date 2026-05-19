//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1276/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1276<F: Float>(t2328: F, t8013: F, t2336: F, t8028: F, t2192: F, t8004: F, t8017: F, t1185: F, t6142: F, t6143: F, t1306: F, t22359: F, t22361: F, t22363: F, t22366: F, t22374: F, t2464: F, t8563: F, t955: F) -> (F, F, F, F, F, F) {
    let t22376 = F::cast_from(0.10389515463408878255e3_f64) * t2328 * t8013;
    let t22378 = F::cast_from(0.17544670867903938621e1_f64) * t8028 * t2336;
    let t22380 = F::new(3.0) * t2192 * t8004;
    let t22382 = F::cast_from(0.31168546390226634765e3_f64) * t2328 * t8017;
    let t22385 = F::new(24.0) * t6142 * t1185 * t6143;
    let t22386 = -F::new(3.0) * t1306 * t2464 * t8563 * t955 + t22359 + t22361 - t22363 - t22366 - t22374 - t22376 - t22378 + t22380 + t22382 - t22385;
    (t22376, t22378, t22380, t22382, t22385, t22386)
}
