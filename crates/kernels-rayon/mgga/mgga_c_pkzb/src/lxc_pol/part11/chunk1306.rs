//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1306/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1306(t31390: f64, t31394: f64, t31397: f64, t31400: f64, t31404: f64, t31407: f64, t31411: f64, t31643: f64, t31647: f64, t31650: f64, t31653: f64, t10106: f64, t300: f64) -> (f64, f64) {
    let t31654 = t31390 - t31394 - t31397 - t31400 + t31404 + t31407 + t31411 - t31643 - t31647 - t31650 - t31653;
    let t31668 = t300 * t10106;
    (t31654, t31668)
}
