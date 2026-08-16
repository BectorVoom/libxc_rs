//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1253/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1253(t26656: f64, t27744: f64, t26395: f64, t26397: f64, t26405: f64, t26408: f64, t26660: f64, t28901: f64, t91769: f64, t91772: f64, t91773: f64, t91776: f64, t91777: f64, t91778: f64, t91781: f64, t95270: f64, t95271: f64, t95272: f64, t95273: f64, t95274: f64) -> f64 {
    let t95275 = 4.0_f64 * t26656;
    let t95276 = t27744 / 8.0_f64;
    let t95277 = t95270 - t91769 - t26395 - t26397 + t91772 + t91773 - t26405 - t26408 + t95271 - t91776 - t95272 + t95273 + t91777 + t95274 + t28901 - t91778 + t95275 - t95276 - t26660 - t91781;
    t95277
}
