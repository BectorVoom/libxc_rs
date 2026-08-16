//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 643/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk643(t7944: f64, t1771: f64, t380: f64, t17: f64, t7760: f64, t62: f64, t66: f64, t401: f64, t77: f64, t408: f64, t428: f64, t3020: f64) -> (f64, f64, f64, f64, f64) {
    let t7945 = 28.0_f64 / 27.0_f64 * t7944;
    let t7946 = t1771 * t380;
    let t7954 = t17 * t7760;
    let t7983 = t62 * t66;
    let t7984 = t77 * t401;
    let t7985 = t7983 * t7984;
    let t7988 = t408 * t428;
    let t7989 = t3020 * t7988;
    (t7945, t7946, t7954, t7985, t7989)
}
