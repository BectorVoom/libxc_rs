//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 602/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk602(t2347: f64, t312: f64, t3886: f64, t4139: f64, t1212: f64, t684: f64, t2874: f64, t1248: f64, t870: f64, t2881: f64, t1250: f64, t1882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4140 = t312 * t2347;
    let t4141 = t4140 * t3886;
    let t4142 = t4139 * t4141;
    let t4145 = t312 * t1212;
    let t4146 = t4145 * t684;
    let t4147 = t2874 * t4146;
    let t4150 = t870 * t1248;
    let t4151 = t4150 * t684;
    let t4152 = t2881 * t4151;
    let t4156 = t1882 * t1250;
    (t4140, t4141, t4142, t4145, t4146, t4147, t4150, t4151, t4152, t4156)
}
