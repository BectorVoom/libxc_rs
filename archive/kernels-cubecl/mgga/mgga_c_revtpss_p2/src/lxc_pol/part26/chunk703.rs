//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 703/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk703<F: Float>(t1358: F, t7492: F, t689: F, t2098: F, t786: F, t1364: F, t7250: F, t7257: F, t7260: F, t7267: F, t7253: F, t7265: F, t7272: F) -> (F, F, F, F, F) {
    let t7493 = t7492 * t1358;
    let t7495 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t7493;
    let t7496 = t786 * t2098;
    let t7498 = F::cast_from(0.9757440539382783019e-2_f64) * t7496 * t1364;
    let t7499 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t7250;
    let t7501 = F::cast_from(0.28582678745379824648e-4_f64) * t7257;
    let t7502 = F::cast_from(0.50820002809285328225e-4_f64) * t7260;
    let t7504 = F::cast_from(0.40015750243531754507e-2_f64) * t7267;
    let t7506 = -t7499 - t7253 / F::cast_from(24.0_f64) - t7501 + t7502 - F::cast_from(0.85748036236139473944e-3_f64) * t7265 - t7504 - F::cast_from(0.34299214494455789578e-2_f64) * t7272;
    (t7493, t7495, t7496, t7498, t7506)
}
