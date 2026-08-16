//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 866/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk866(t1113: f64, t2426: f64, t1127: f64, t9681: f64, t13452: f64, t2379: f64, t13448: f64, t13407: f64, t3785: f64, t122: f64, t13402: f64, t3751: f64, t709: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13491 = t2426 * t1113;
    let t13495 = t9681 * t1127;
    let t13499 = t2379 * t13452;
    let t13502 = t2379 * t13448;
    let t13505 = t3785 * t13407;
    let t13508 = t1127 * t122;
    let t13509 = t13508 * t13402;
    let t13515 = t3751 * t709;
    (t13491, t13495, t13499, t13502, t13505, t13509, t13515)
}
