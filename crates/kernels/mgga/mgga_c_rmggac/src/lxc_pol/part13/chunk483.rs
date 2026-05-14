//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 483/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk483<F: Float>(t4366: F, t4368: F, t4370: F, t1412: F, t2: F, t428: F, t1372: F, t980: F, t973: F, t421: F, t155: F, t4324: F, t4328: F, t4361: F, t4365: F, t5435: F, t5445: F, t5447: F, t5449: F, t5451: F) -> (F, F, F, F, F, F, F, F) {
    let t5459 = 16.0 * t4366;
    let t5460 = 4.0 * t4368;
    let t5461 = 4.0 * t4370;
    let t5462 = t1412 * t2;
    let t5464 = 0.36622894612013090108e-3 * t5462 * t428;
    let t5465 = t1372 * t980;
    let t5466 = 0.11696447245269292414e1 * t5465;
    let t5467 = t1372 * t973;
    let t5468 = 0.5848223622634646207e0 * t5467;
    let t5469 = t1412 * t421;
    let t5471 = 2.0 * t155 * t5469;
    let t5472 = t5435 + t4361 - t4365 + t5445 + t5447 + t5449 - t5451 + t4324 - t5459 - t5460 - t5461 + t4328 - t5464 + t5466 - t5468 + t5471;
    (t5459, t5460, t5461, t5464, t5466, t5468, t5471, t5472)
}
