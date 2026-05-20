//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1379/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1379<F: Float>(t12772: F, t5401: F, t3625: F, t1214: F, t5341: F, t5332: F, t3720: F, t1250: F, t5346: F, t16725: F, t5312: F, t16729: F) -> (F, F, F, F, F, F) {
    let t17451 = t12772 * t5401;
    let t17453 = F::cast_from(0.19055119163586549765e-3_f64) * t3625 * t17451;
    let t17454 = t5341 * t1214;
    let t17455 = t5332 * t17454;
    let t17456 = t3720 * t17455;
    let t17459 = t1250 * t1214;
    let t17460 = t5346 * t17459;
    let t17461 = t3720 * t17460;
    let t17464 = t5312 * t16725;
    let t17467 = t5312 * t16729;
    (t17453, t17454, t17456, t17461, t17464, t17467)
}
