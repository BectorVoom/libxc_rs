//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 828/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk828(t11604: f64, t3434: f64, t2210: f64, t3440: f64, t3439: f64, t3425: f64, t9419: f64, t12278: f64, t144: f64, t1651: f64, t3419: f64, t1643: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12999 = t3434 * t11604;
    let t13000 = t2210 * t12999;
    let t13003 = t3440 * t11604;
    let t13004 = t3439 * t13003;
    let t13007 = t9419 * t3425;
    let t13010 = t144 * t12278;
    let t13013 = t3419 * t1651;
    let t13014 = t2210 * t13013;
    let t13017 = t3419 * t1643;
    (t13000, t13004, t13007, t13010, t13014, t13017)
}
