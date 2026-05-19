//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 462/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk462<F: Float>(t1412: F, t2: F, t428: F, t1372: F, t980: F, t973: F, t421: F, t155: F, t1439: F, t453: F, t1156: F, t592: F) -> (F, F, F, F, F, F) {
    let t5462 = t1412 * t2;
    let t5464 = F::cast_from(0.36622894612013090108e-3_f64) * t5462 * t428;
    let t5465 = t1372 * t980;
    let t5467 = t1372 * t973;
    let t5469 = t1412 * t421;
    let t5471 = F::new(2.0) * t155 * t5469;
    let t5477 = t1439 * t453;
    let t5480 = t592 * t1156;
    (t5464, t5465, t5467, t5471, t5477, t5480)
}
