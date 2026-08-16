//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 651/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk651(t2205: f64, t7807: f64, t446: f64, t1651: f64, t558: f64, t1969: f64, t2075: f64, t379: f64, t1642: f64, t525: f64, t1643: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9038 = t2205 * t7807;
    let t9039 = t446 * t9038;
    let t9041 = t1651 * t558;
    let t9042 = t1969 * t9041;
    let t9043 = t446 * t9042;
    let t9045 = t379 * t2075;
    let t9046 = t1969 * t9045;
    let t9047 = t446 * t9046;
    let t9049 = t1642 * t525;
    let t9050 = t1643 * t558;
    let t9051 = t9049 * t9050;
    let t9052 = t446 * t9051;
    (t9038, t9039, t9041, t9042, t9043, t9045, t9046, t9047, t9049, t9050, t9051, t9052)
}
