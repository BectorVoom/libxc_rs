//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 707/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk707(t3447: f64, t8392: f64, t3436: f64, t3426: f64, t3431: f64, t1882: f64, t3567: f64, t12001: f64, t3471: f64, t3467: f64, t12306: f64, t12308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12975 = 2.0_f64 / 27.0_f64 * t8392 * t3447;
    let t13040 = 4.0_f64 / 27.0_f64 * t8392 * t3436;
    let t13042 = 2.0_f64 / 27.0_f64 * t8392 * t3426;
    let t13049 = 2.0_f64 / 27.0_f64 * t8392 * t3431;
    let t13062 = 2.0_f64 / 9.0_f64 * t1882 * t3567;
    let t13075 = t12001 * t3471;
    let t13084 = 2.0_f64 / 27.0_f64 * t1882 * t3467;
    let t13100 = 2.0_f64 / 9.0_f64 * t12306;
    let t13101 = 4.0_f64 / 9.0_f64 * t12308;
    (t12975, t13040, t13042, t13049, t13062, t13075, t13084, t13100, t13101)
}
