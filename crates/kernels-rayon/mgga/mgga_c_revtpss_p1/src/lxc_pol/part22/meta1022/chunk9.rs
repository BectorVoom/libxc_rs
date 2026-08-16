//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3570/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3570(t20400: f64, t3543: f64, t1765: f64, t57861: f64, t16784: f64, t5207: f64, t12248: f64, t3385: f64, t6439: f64, t3367: f64, t60717: f64, t1120: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68243 = 0.17315859105681463759e2_f64 * t20400 * t3543;
    let t68245 = 0.11696447245269292414e1_f64 * t57861 * t1765;
    let t68247 = 0.69263436422725855034e2_f64 * t16784 * t5207;
    let t68250 = 24.0_f64 * t12248 * t6439 * t3385;
    let t68251 = t3367 * t60717;
    let t68253 = t128 * t1120 * t68251;
    (t68243, t68245, t68247, t68250, t68251, t68253)
}
