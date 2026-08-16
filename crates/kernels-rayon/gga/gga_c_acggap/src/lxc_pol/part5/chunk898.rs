//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 898/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk898(t13326: f64, t150: f64, t164: f64, t177: f64, t968: f64, t977: f64, t151: f64, t161: f64, t7510: f64, t3171: f64, t3372: f64, t171: f64, t368: f64) -> (f64, f64, f64, f64, f64) {
    let t13330 = 0.21437009059034868486e-3_f64 * t13326 * t150 * t164 * t177;
    let t13332 = t977 * t968;
    let t13337 = 0.28974367305964659283e0_f64 * t151 * t161 * t7510 * t177;
    let t13344 = t3372 * t3171;
    let t13364 = t171 * t368;
    (t13330, t13332, t13337, t13344, t13364)
}
