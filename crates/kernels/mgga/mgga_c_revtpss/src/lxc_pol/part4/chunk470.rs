//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 470/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk470<F: Float>(t1598: F, t1612: F, t1614: F, t1622: F, t1627: F, t1634: F, t300: F, t311: F, t946: F, t965: F, t1633: F, t964: F, t973: F) -> (F, F, F) {
    let t1638 = t300 * (-F::new(0.310907e-1) * t1614 * t311 + F::new(1.0) * t946 * t1622 + t1598 - t1612 - F::cast_from(0.19751673498613801407e-1_f64) * t1627 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t1634);
    let t1640 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t1627;
    let t1642 = t964 * t1633 * t973;
    (t1638, t1640, t1642)
}
