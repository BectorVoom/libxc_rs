//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3043/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3043(t76396: f64, t1733: f64, t68947: f64, t20629: f64, t5105: f64, t16835: f64, t6471: f64, t20448: f64, t5063: f64, t58466: f64, t6474: f64, t24262: f64, t44101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81123 = -t76396;
    let t81128 = 3.0_f64 * t68947 * t1733;
    let t81130 = 3.0_f64 * t20629 * t5105;
    let t81132 = 3.0_f64 * t16835 * t6471;
    let t81134 = 3.0_f64 * t5063 * t20448;
    let t81136 = 0.48245938496077605201e2_f64 * t58466 * t6474;
    let t81138 = 0.96491876992155210402e2_f64 * t44101 * t24262;
    (t81123, t81128, t81130, t81132, t81134, t81136, t81138)
}
