//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2491/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2491(t13069: f64, t3704: f64, t12941: f64, t3708: f64, t12948: f64, t13058: f64, t12937: f64, t3172: f64, t3711: f64, t13080: f64, t5384: f64, t1231: f64, t12898: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44278 = t13069 * t3704;
    let t44280 = t3708 * t12941;
    let t44283 = t13058 * t12948;
    let t44286 = t3711 * t3172 * t12937;
    let t44289 = t5384 * t3172 * t13080;
    let t44291 = t1231 * t12898;
    (t44278, t44280, t44283, t44286, t44289, t44291)
}
