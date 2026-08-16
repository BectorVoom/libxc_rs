//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 539/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk539(t2218: f64, t2221: f64, t2225: f64, t2232: f64, t1406: f64, t604: f64, t1437: f64, t645: f64, t1409: f64, t607: f64) -> (f64, f64, f64, f64) {
    let t3951 = -t2218 - 0.78e0_f64 * t2221 - 0.578e2_f64 * t2225 + t2232;
    let t3953 = t1406 * t604;
    let t3958 = t1437 * t645;
    let t3961 = t607 * t1409;
    (t3951, t3953, t3958, t3961)
}
