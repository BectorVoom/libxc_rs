//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 510/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk510<F: Float>(t1568: F, t212: F, t780: F, t689: F, t1569: F, t786: F, t789: F, t1469: F, t80: F, t83: F, t1544: F, t221: F, t2675: F, t2674: F, t1558: F, t243: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4321 = t212 * t1568;
    let t4322 = t4321 * t780;
    let t4323 = t689 * t4322;
    let t4325 = t786 * t1569;
    let t4326 = t4325 * t789;
    let t4328 = t80 * t1469;
    let t4335 = t83 * t1469;
    let t4349 = t2675 * t221 * t1544;
    let t4350 = t2674 * t4349;
    let t4352 = t243 * t1558;
    (t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4349, t4350, t4352)
}
