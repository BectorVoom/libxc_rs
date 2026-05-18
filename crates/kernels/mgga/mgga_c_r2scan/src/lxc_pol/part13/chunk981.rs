//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 981/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk981<F: Float>(t11518: F, t3276: F, t3262: F, t106: F, t2530: F, t97: F) -> (F, F, F, F) {
    let t11519 = t3276 * t11518;
    let t11520 = t3262 * t11519;
    let t11521 = F::new(15.0) / F::new(16.0) * t11520;
    let t11523 = t97 * t106 * t2530;
    (t11519, t11520, t11521, t11523)
}
