//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 674/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk674<F: Float>(t57: F, t1469: F, t83: F, t4186: F, t606: F, t770: F, t4334: F, zeta_threshold: F) -> (F, F) {
    let t155 = t57 <= zeta_threshold;
    let t4335 = t83 * t1469;
    let t4341 = piecewise3::<f64>(t155, F::new(0.0), -F::new(2.0) / F::new(9.0) * t4335 * t606 - F::new(2.0) / F::new(3.0) * t770 * t4186);
    let t4343 = t4334 / F::new(2.0) + t4341 / F::new(2.0);
    (t4335, t4343)
}
