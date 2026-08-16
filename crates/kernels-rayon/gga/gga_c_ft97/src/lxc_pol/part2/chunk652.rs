//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 652/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk652(t10: f64, t144: f64, t3050: f64, t1984: f64, t378: f64, t2214: f64, t8392: f64, t2225: f64, t582: f64, t597: f64, t1882: f64, t2159: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9071 = t10 * t3050 * t144;
    let t9072 = 14.0_f64 / 81.0_f64 * t9071;
    let t9073 = t378 * t1984;
    let t9090 = t8392 * t2214;
    let t9097 = t8392 * t2225;
    let t9099 = t582 * t597;
    let t9106 = t1882 * t2159;
    (t9071, t9072, t9073, t9090, t9097, t9099, t9106)
}
