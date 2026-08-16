//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 604/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk604(t4176: f64, t840: f64, t871: f64, t1248: f64, t875: f64, t2843: f64, t296: f64, t1255: f64, t684: f64, t835: f64, t1234: f64, t2755: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4178 = t840 * t871 * t4176;
    let t4181 = t1248 * t875;
    let t4182 = t2843 * t4181;
    let t4183 = t296 * t4182;
    let t4188 = t835 * t1255 * t684;
    let t4191 = t2755 * t1234;
    (t4178, t4181, t4182, t4183, t4188, t4191)
}
