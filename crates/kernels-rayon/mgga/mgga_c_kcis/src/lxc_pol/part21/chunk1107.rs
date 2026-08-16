//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1107/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1107(t11178: f64, t2192: f64, t7768: f64, t7784: f64, t3638: f64, t7779: f64, t2197: f64, t26673: f64, t26677: f64, t26708: f64, t26712: f64, t26721: f64, t26955: f64, t26957: f64, t26960: f64, t26963: f64, t26966: f64, t26974: f64, t26977: f64, t7772: f64, t7791: f64) -> (f64, f64, f64, f64) {
    let t26982 = t11178 * t2192;
    let t26985 = t7768 * t7784;
    let t26987 = t3638 * t7779;
    let t26991 = -0.61905925925925925925e-2_f64 * t26673 - 0.23214722222222222222e-2_f64 * t26677 + 0.30918233506944444444e-4_f64 * t26955 * t26957 + 0.23168402777777777778e-3_f64 * t26960 * t26963 + 0.61782407407407407408e-3_f64 * t26966 * t7791 + 0.23168402777777777778e-3_f64 * t26960 * t26957 + t26974 - 0.13913205078125e-3_f64 * t7772 * t26977 + 0.11607361111111111111e-2_f64 * t26708 + 0.19345601851851851852e-2_f64 * t26712 - 0.34752604166666666667e-3_f64 * t26982 * t2197 - 0.23168402777777777778e-3_f64 * t26985 + 0.18534722222222222222e-2_f64 * t26987 * t2197 - 0.38691203703703703703e-3_f64 * t26721;
    (t26982, t26985, t26987, t26991)
}
