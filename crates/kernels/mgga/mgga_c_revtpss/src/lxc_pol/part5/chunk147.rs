//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 147/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk147<F: Float>(t439: F, t448: F, t300: F, t424: F, t426: F, t435: F, t406: F) -> (F, F, F, F, F) {
    let t449 = t439 * t448;
    let t452 = t300 * (-F::new(0.310907e-1) * t426 * t435 + t424 - F::cast_from(0.19751673498613801407e-1_f64) * t449);
    let t454 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t449;
    let t456 = F::new(1.0) + F::new(0.25e-1) * t406;
    let t458 = F::new(1.0) + F::new(0.4445e-1) * t406;
    let t459 = F::new(1.0) / t458;
    (t452, t454, t456, t458, t459)
}
