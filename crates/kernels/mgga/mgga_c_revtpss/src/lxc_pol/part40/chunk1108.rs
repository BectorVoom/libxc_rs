//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1108/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1108<F: Float>(t45: F, t57: F, t13312: F, t14401: F, t14404: F, t2251: F, t2258: F, t4377: F, t606: F, t78: F, t10457: F, t1469: F, t2382: F, t4186: F, t4384: F, t81: F, t162: F, t187: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t14412 = piecewise3(t151, 0.0, -8.0 / 27.0 * t14401 * t2251 + 8.0 / 9.0 * t14404 * t606 + 4.0 / 9.0 * t4377 * t2258 + 4.0 / 3.0 * t78 * t13312);
    let t14413 = t10457 * t1469;
    let t14416 = t2382 * t4186;
    let t14424 = piecewise3(t155, 0.0, 8.0 / 27.0 * t14413 * t2251 + 8.0 / 9.0 * t14416 * t606 + 4.0 / 9.0 * t4384 * t2258 - 4.0 / 3.0 * t81 * t13312);
    let t14425 = t14412 + t14424;
    let t14426 = t14425 * t162;
    let t14428 = 0.19751673498613801407e-1 * t14426 * t187;
    (t14425, t14428)
}
