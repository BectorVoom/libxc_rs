//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 471/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk471(t397: f64, t3979: f64, t403: f64, t396: f64, t172: f64, t301: f64, t342: f64, t142: f64, t416: f64, t10: f64, t1337: f64, t1232: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3981 = t397 * t3979 * t403;
    let t3983 = 0.11993859144118211475e-1_f64 * t396 * t3981;
    let t4007 = t342 * t172 * t301;
    let t4008 = 0.23744444444444444444e-1_f64 * t4007;
    let t4009 = t142 * t416;
    let t4013 = t10 * t1337;
    let t4029 = t1232 * t357;
    (t3981, t3983, t4007, t4008, t4009, t4013, t4029)
}
