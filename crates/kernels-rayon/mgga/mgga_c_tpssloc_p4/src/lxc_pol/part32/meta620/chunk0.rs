//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2024/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2024(t7365: f64, t85660: f64, t131: f64, t467: f64, t50: f64, t82510: f64, t10469: f64, t461: f64, t11721: f64, t3032: f64, t3508: f64, t7368: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85952 = t85660 * t7365;
    let t85963 = t50 * t82510 * t131 * t467;
    let t85964 = t461 * t10469;
    let t85966 = t3032 * t11721;
    let t85972 = t3032 * t3508;
    let t85986 = t85660 * t7368;
    (t85952, t85963, t85964, t85966, t85972, t85986)
}
