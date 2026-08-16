//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1362/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1362(t27219: f64, t8526: f64, t22574: f64, t25988: f64, t36740: f64, t26168: f64, t8607: f64, t31747: f64, t4028: f64, t26149: f64, t26161: f64, t33221: f64, t92200: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121169 = 2.0_f64 * t8526 * t27219;
    let t121174 = 3.0_f64 * t22574 * t36740 * t25988;
    let t121177 = 3.0_f64 * t8607 * t26168;
    let t121179 = 2.0_f64 * t4028 * t31747;
    let t121181 = t8607 * t26149;
    let t121184 = 2.0_f64 * t26161 * t92200 * t33221;
    (t121169, t121174, t121177, t121179, t121181, t121184)
}
