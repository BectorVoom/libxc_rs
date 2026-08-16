//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1884/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1884(t26414: f64, t6976: f64, t22633: f64, t5345: f64, t1992: f64, t1799: f64, t562: f64) -> (f64, f64, f64, f64, f64) {
    let t26415 = t6976 * t26414;
    let t26416 = t22633 * t26415;
    let t26418 = t6976 * t5345;
    let t26419 = t1992 * t26418;
    let t26421 = t562 * t1799;
    (t26415, t26416, t26418, t26419, t26421)
}
