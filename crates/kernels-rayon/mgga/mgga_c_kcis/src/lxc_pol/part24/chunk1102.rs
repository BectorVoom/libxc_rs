//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1102/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1102(t28059: f64, t8069: f64, t28045: f64, t8072: f64, t5047: f64, t6486: f64, t26896: f64, t6693: f64, t7748: f64, t6613: f64, t5077: f64, t6496: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29047 = t28059 * t8069;
    let t29049 = t28045 * t8072;
    let t29051 = t5047 * t6486;
    let t29052 = t26896 * t29051;
    let t29054 = t7748 * t6693;
    let t29056 = t5047 * t6613;
    let t29057 = t7748 * t29056;
    let t29059 = t5077 * t6496;
    (t29047, t29049, t29051, t29052, t29054, t29056, t29057, t29059)
}
