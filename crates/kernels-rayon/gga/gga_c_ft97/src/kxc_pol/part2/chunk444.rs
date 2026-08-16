//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 444/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk444(t2: f64, t2347: f64, t2349: f64, t2486: f64, t665: f64, t675: f64) -> (f64, f64, f64) {
    let t2487 = t2 * t2347;
    let t2488 = t2487 * t2349;
    let t2489 = t2486 * t2488;
    let t2492 = t665 * t675;
    (t2488, t2489, t2492)
}
