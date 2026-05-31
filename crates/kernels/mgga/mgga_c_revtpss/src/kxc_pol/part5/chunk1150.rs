//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1150/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1150<F: Float>(t45: F, t57: F, t5819: F, t633: F, t5825: F, t80: F, t18281: F, t4186: F, t4328: F, t606: F, t766: F, t637: F, t83: F, t4335: F, t770: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t18367 = t633 * t5819;
    let t18372 = t80 * t5825;
    let t18378 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t18367 * t606 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4328 * t4186 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18372 * t606 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t18281);
    let t18379 = t637 * t5819;
    let t18384 = t83 * t5825;
    let t18390 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t18379 * t606 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4335 * t4186 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18384 * t606 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t18281);
    (t18378, t18390)
}
