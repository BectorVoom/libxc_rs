//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 884/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk884(t33334: f64, t533: f64, t1390: f64, t1983: f64, t7802: f64, t8526: f64, t1799: f64, t2018: f64, t24432: f64, t22574: f64, t7685: f64, t8644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33335 = t533 * t33334;
    let t33336 = t33335 * t1390;
    let t33337 = t1983 * t33336;
    let t33345 = 2.0_f64 * t8526 * t7802;
    let t33357 = t2018 * t1799;
    let t33358 = t24432 * t33357;
    let t33360 = 3.0_f64 * t22574 * t33358;
    let t33361 = t7685 * t8644;
    (t33335, t33336, t33337, t33345, t33357, t33358, t33360, t33361)
}
