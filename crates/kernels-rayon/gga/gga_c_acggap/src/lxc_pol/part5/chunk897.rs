//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 897/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk897(t968: f64, t981: f64, t974: f64, t177: f64, t3646: f64, t414: f64, t973: f64, t980: f64, t3559: f64, t377: f64, t107: f64, t118: f64, t11805: f64, t11820: f64, t4: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13308 = t981 * t968;
    let t13310 = t974 * t968;
    let t13314 = 0.40015750243531754508e-2_f64 * t3646 * t414 * t177;
    let t13317 = 0.34013387707001991332e-1_f64 * t980 * t973 * t177;
    let t13320 = 0.15117061203111996148e0_f64 * t377 * t3559 * t177;
    let t13326 = (0.43209876543209876543e0_f64 * t4 * t11805 * t107 + 0.15432407407407407408e0_f64 * t11820) * t118;
    (t13308, t13310, t13314, t13317, t13320, t13326)
}
