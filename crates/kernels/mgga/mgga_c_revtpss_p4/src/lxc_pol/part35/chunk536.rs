//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 536/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk536<F: Float>(t5: F, t1497: F, t2247: F, t4173: F, t5812: F, t5816: F, t5872: F, t603: F, t91: F, t117: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t5876 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) * t1497 * t4173 + F::cast_from(20.0_f64) * t2247 * t5816 + t5812 * t91 - F::cast_from(4.0_f64) * t5872 * t603);
    let t5877 = t5876 * t117;
    (t5876, t5877)
}
