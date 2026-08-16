//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1180/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1180<F: Float>(t45: F, t4321: F, t780: F, t689: F, t1569: F, t786: F, t789: F, t1469: F, t80: F, t4186: F, t606: F, t766: F, t83: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t4322 = t4321 * t780;
    let t4323 = t689 * t4322;
    let t4325 = t786 * t1569;
    let t4326 = t4325 * t789;
    let t4328 = t80 * t1469;
    let t4334 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4328 * t606 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t4186);
    let t4335 = t83 * t1469;
    (t4322, t4323, t4325, t4326, t4328, t4334, t4335)
}
