//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1084/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1084<F: Float>(t140: F, t5368: F, t1222: F, t3624: F, t5436: F, t12772: F, t5401: F, t3625: F, t1214: F, t1250: F, t3698: F, t5047: F, t1012: F, t13026: F, t1263: F, t5245: F) -> (F, F, F, F, F, F, F) {
    let t17445 = t140 * t5368;
    let t17447 = t1222 * t17445 / 432.0;
    let t17448 = t5436 * t3624;
    let t17451 = t12772 * t5401;
    let t17453 = 0.19055119163586549765e-3 * t3625 * t17451;
    let t17459 = t1250 * t1214;
    let t17471 = t140 * t3698;
    let t17472 = t17471 * t5047;
    let t17474 = t1222 * t17472 / 324.0;
    let t17475 = t1012 * t13026;
    let t17500 = t1263 * t5245;
    (t17447, t17448, t17453, t17459, t17474, t17475, t17500)
}
