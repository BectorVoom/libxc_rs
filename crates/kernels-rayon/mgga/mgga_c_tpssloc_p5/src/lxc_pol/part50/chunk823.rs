//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 823/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk823(t1873: f64, t6517: f64, t8319: f64, t88: f64, t1268: f64, t8326: f64, t8313: f64, t191: f64, t1980: f64, t192: f64) -> (f64, f64, f64, f64) {
    let t8441 = t6517 * t1873;
    let t8444 = 2.0_f64 * t88 * t8319;
    let t8445 = t1268 * t8326;
    let t8446 = 2.0_f64 * t8445;
    let t8447 = t8313 + 4.0_f64 * t8441 + t8444 + t8446;
    let t8449 = t1980 * t191;
    let t8450 = t8449 * t192;
    (t8446, t8447, t8449, t8450)
}
