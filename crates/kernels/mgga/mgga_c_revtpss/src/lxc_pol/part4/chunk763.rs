//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 763/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk763<F: Float>(t45: F, t57: F, t1522: F, t2398: F, t1568: F, t212: F, t780: F, t689: F, t1569: F, t786: F, t789: F, t1469: F, t80: F, t4186: F, t606: F, t766: F, t83: F, t770: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t4316 = 4.0 * t2398 * t1522;
    let t4321 = t212 * t1568;
    let t4322 = t4321 * t780;
    let t4323 = t689 * t4322;
    let t4325 = t786 * t1569;
    let t4326 = t4325 * t789;
    let t4328 = t80 * t1469;
    let t4334 = piecewise3(t151, 0.0, -2.0 / 9.0 * t4328 * t606 + 2.0 / 3.0 * t766 * t4186);
    let t4335 = t83 * t1469;
    let t4341 = piecewise3(t155, 0.0, -2.0 / 9.0 * t4335 * t606 - 2.0 / 3.0 * t770 * t4186);
    (t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4334, t4335, t4341)
}
