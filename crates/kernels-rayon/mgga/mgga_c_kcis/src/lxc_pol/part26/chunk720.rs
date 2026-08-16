//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 720/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk720(t7921: f64, t2260: f64, t7926: f64, t7929: f64, t7933: f64, t7936: f64, t7964: f64, t7968: f64, t7971: f64, t7976: f64, t7978: f64, t7981: f64, t7986: f64) -> (f64, f64) {
    let t7991 = 0.11607361111111111111e-2_f64 * t7921;
    let t7996 = -0.34752604166666666667e-3_f64 * t7964 * t2260 + 0.46377350260416666667e-4_f64 * t7968 * t7971 - t7976 - 0.11584201388888888889e-3_f64 * t7978 * t7981 + 0.34752604166666666667e-3_f64 * t7978 * t7986 + 0.34752604166666666667e-3_f64 * t7978 * t7971 + t7991 + 0.11607361111111111111e-2_f64 * t7926 + 0.17411041666666666666e-2_f64 * t7929 - 0.17411041666666666666e-2_f64 * t7933 + 0.11607361111111111111e-2_f64 * t7936;
    (t7991, t7996)
}
