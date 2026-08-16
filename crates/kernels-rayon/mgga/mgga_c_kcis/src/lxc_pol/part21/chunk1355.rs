//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1355/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1355(t11020: f64, t26955: f64, t26960: f64, t26977: f64, t28096: f64, t28204: f64, t2829: f64, t2845: f64, t3515: f64, t95884: f64, t95887: f64, t96739: f64, t96857: f64, t96977: f64, t96980: f64, t96993: f64, t96995: f64, t96999: f64) -> f64 {
    let t97006 = -0.92754700520833333333e-4_f64 * t26955 * t96739 + t96977 - t96980 - 0.46377350260416666666e-4_f64 * t26955 * t96857 + 0.11584201388888888889e-3_f64 * t26960 * t3515 * t28096 * t2829 + 0.15445601851851851852e-3_f64 * t26960 * t11020 * t28096 * t2845 + t96993 + 0.11584201388888888889e-3_f64 * t26960 * t96995 + 0.15445601851851851852e-3_f64 * t26960 * t96999 - 0.11607361111111111111e-2_f64 * t95884 + 0.61905925925925925925e-2_f64 * t95887 - 0.13913205078125e-3_f64 * t28204 * t26977;
    t97006
}
