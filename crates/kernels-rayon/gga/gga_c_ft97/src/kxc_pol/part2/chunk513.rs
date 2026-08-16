//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 513/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk513(t3056: f64, t35: f64, t374: f64, t1594: f64, t3037: f64, t25: f64, t938: f64, t373: f64, t383: f64, t408: f64, t401: f64, t64: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3057 = t3056 * t35;
    let t3058 = t374 * t3057;
    let t3061 = t1594 * t3037;
    let t3064 = t938 * t25;
    let t3065 = t373 * t383;
    let t3066 = t3065 * t35;
    let t3067 = t3064 * t3066;
    let t3070 = t408 * t938;
    let t3071 = t3070 * t401;
    let t3076 = t64 * t66;
    (t3057, t3058, t3061, t3064, t3066, t3067, t3070, t3071, t3076)
}
