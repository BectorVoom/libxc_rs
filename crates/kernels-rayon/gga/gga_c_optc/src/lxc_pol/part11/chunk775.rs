//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 775/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk775(t4570: f64, t6547: f64, t1310: f64, t3563: f64, t1916: f64, t4758: f64, t188: f64, t1975: f64, t4727: f64, t4579: f64, t75: f64, t603: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13020 = t6547 * t4570;
    let t13050 = t1310 * t3563;
    let t13053 = t1916 * t4758;
    let t13054 = t188 * t13053;
    let t13056 = t4727 * t1975;
    let t13061 = t4579 * t75;
    let t13062 = t13061 * t603;
    (t13020, t13050, t13053, t13054, t13056, t13061, t13062)
}
