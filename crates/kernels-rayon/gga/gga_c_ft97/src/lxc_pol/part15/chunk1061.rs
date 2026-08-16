//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1061/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1061(t20655: f64, t925: f64, t1017: f64, t20045: f64, t11755: f64, t12796: f64, t17259: f64, t1985: f64, t2097: f64, t2102: f64, t40337: f64, t40485: f64, t462: f64, t4714: f64, t582: f64, t78164: f64, t78251: f64, t86023: f64, t86027: f64, t86031: f64, t86035: f64, t86039: f64, t86043: f64, t86121: f64, t86876: f64, t9016: f64, t9224: f64) -> (f64, f64, f64) {
    let t86902 = t925 * t20655;
    let t86906 = t20045 * t1017;
    let t86933 = -4.0_f64 / 3.0_f64 * t78251 + t40485 + 8.0_f64 / 3.0_f64 * t11755 * t12796 * t86876 + 8.0_f64 * t462 * t582 * t86027 + 2.0_f64 * t462 * t582 * t86039 + 4.0_f64 / 3.0_f64 * t462 * t2102 * t86902 + 4.0_f64 / 3.0_f64 * t462 * t2102 * t86906 - 80.0_f64 / 81.0_f64 * t462 * t40337 * t86121 - 36.0_f64 * t462 * t9016 * t17259 * t4714 - t462 * t582 * t86031 / 3.0_f64 + 8.0_f64 * t462 * t1985 * t78164 * t1017 + 40.0_f64 / 9.0_f64 * t462 * t9224 * t86023 - 8.0_f64 * t462 * t2097 * t86043 - 2.0_f64 / 3.0_f64 * t462 * t2097 * t86035;
    (t86902, t86906, t86933)
}
