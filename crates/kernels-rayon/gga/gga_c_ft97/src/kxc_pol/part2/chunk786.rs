//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 786/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk786(t12318: f64, t446: f64, t11050: f64, t569: f64, t11059: f64, t2205: f64, t1882: f64, t3339: f64, t3408: f64, t358: f64, t363: f64, t1969: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12319 = t446 * t12318;
    let t12321 = t569 * t11050;
    let t12322 = t446 * t12321;
    let t12324 = t2205 * t11059;
    let t12325 = t446 * t12324;
    let t12327 = t1882 * t3339;
    let t12328 = t12327 / 27.0_f64;
    let t12329 = t3408 * t358;
    let t12330 = t12329 * t363;
    let t12331 = t1969 * t12330;
    (t12319, t12322, t12325, t12327, t12328, t12330, t12331)
}
