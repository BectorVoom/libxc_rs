//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1130/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1130<F: Float>(t140: F, t5368: F, t1222: F, t3624: F, t5436: F, t12772: F, t5401: F, t3625: F, t1214: F, t1250: F, t3698: F, t5047: F) -> (F, F, F, F, F) {
    let t17445 = t140 * t5368;
    let t17447 = t1222 * t17445 / F::new(432.0);
    let t17448 = t5436 * t3624;
    let t17451 = t12772 * t5401;
    let t17453 = F::new(0.19055119163586549765e-3) * t3625 * t17451;
    let t17459 = t1250 * t1214;
    let t17471 = t140 * t3698;
    let t17472 = t17471 * t5047;
    (t17447, t17448, t17453, t17459, t17472)
}
