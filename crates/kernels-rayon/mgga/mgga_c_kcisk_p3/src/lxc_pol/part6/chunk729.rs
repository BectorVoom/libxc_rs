//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 729/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk729(t1319: f64, t6174: f64, t301: f64, t342: f64, t969: f64, t119: f64, t416: f64, t1337: f64, t142: f64, t10: f64, t3529: f64, t1265: f64, t4125: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13504 = t6174 * t1319;
    let t13522 = t342 * t969 * t301;
    let t13523 = 0.55403703703703703703e-1_f64 * t13522;
    let t13524 = t119 * t416;
    let t13528 = t142 * t1337;
    let t13538 = t10 * t3529;
    let t13561 = 1.0_f64 / t4125 / t1265;
    (t13504, t13522, t13523, t13524, t13528, t13538, t13561)
}
