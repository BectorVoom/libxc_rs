//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 956/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk956(t12475: f64, t3067: f64, t11453: f64, t4284: f64, t1125: f64, t3096: f64, t9199: f64, t9187: f64, t9684: f64, t3028: f64, t4212: f64, t140: f64, t4227: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12477 = t3067 * t12475 / 3456.0_f64;
    let t12478 = t11453 * t4284;
    let t12480 = t1125 * t12478 / 1728.0_f64;
    let t12490 = t3096 * t9199;
    let t12510 = t9684 * t9187;
    let t12530 = t4212 * t3028 / 162.0_f64;
    let t12535 = t140 * t4227;
    (t12477, t12480, t12490, t12510, t12530, t12535)
}
