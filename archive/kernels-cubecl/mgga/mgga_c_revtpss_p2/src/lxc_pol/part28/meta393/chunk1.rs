//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1486/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1486<F: Float>(t45: F, t10446: F, t1469: F, t2375: F, t4186: F, t13312: F, t2251: F, t2258: F, t4377: F, t606: F, t78: F, t10457: F, t2382: F, zeta_threshold: F) -> (F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t14401 = t10446 * t1469;
    let t14404 = t2375 * t4186;
    let t14412 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t14401 * t2251 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t14404 * t606 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4377 * t2258 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t13312);
    let t14413 = t10457 * t1469;
    let t14416 = t2382 * t4186;
    (t14412, t14413, t14416)
}
