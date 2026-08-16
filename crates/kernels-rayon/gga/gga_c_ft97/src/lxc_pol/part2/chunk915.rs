//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 915/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk915(t258: f64, t3821: f64, t684: f64, t2599: f64, t1160: f64, t2492: f64, t2602: f64, t265: f64, t9895: f64, t13757: f64, t3842: f64, t10007: f64) -> (f64, f64, f64, f64) {
    let t14154 = t258 * t3821;
    let t14155 = t14154 * t684;
    let t14156 = t2599 * t14155;
    let t14159 = t2492 * t1160;
    let t14160 = t14159 * t2602;
    let t14163 = t9895 * t265;
    let t14164 = t14163 * t13757;
    let t14167 = t3842 * t684;
    let t14168 = t10007 * t14167;
    (t14156, t14160, t14164, t14168)
}
