//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1360/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1360(t121007: f64, t1874: f64, t27188: f64, t6535: f64, t31304: f64, t7688: f64, t31537: f64, t7796: f64, t31540: f64, t27163: f64, t8526: f64, t119832: f64, t26161: f64, t26558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121009 = 2.0_f64 * t121007 * t1874;
    let t121019 = 2.0_f64 * t27188 * t6535;
    let t121132 = 3.0_f64 * t31304 * t7688;
    let t121134 = 2.0_f64 * t31537 * t7796;
    let t121136 = 2.0_f64 * t31540 * t7796;
    let t121138 = 2.0_f64 * t8526 * t27163;
    let t121142 = 2.0_f64 * t26161 * t26558 * t119832;
    (t121009, t121019, t121132, t121134, t121136, t121138, t121142)
}
