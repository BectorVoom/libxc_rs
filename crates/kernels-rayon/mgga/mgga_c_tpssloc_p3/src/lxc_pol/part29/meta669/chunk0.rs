//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2235/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2235(t22844: f64, t6976: f64, t22828: f64, t7708: f64, t16391: f64, t26309: f64, t5259: f64, t80820: f64, t16265: f64, t22833: f64, t5293: f64, t80816: f64) -> (f64, f64, f64, f64, f64) {
    let t91208 = t22844 * t6976;
    let t91210 = t91208 * t7708 * t22828;
    let t91212 = t26309 * t16391;
    let t91214 = t80820 * t5259;
    let t91215 = 7.0_f64 / 288.0_f64 * t91214;
    let t91216 = t22833 * t16265;
    let t91218 = t80816 * t5293;
    (t91210, t91212, t91215, t91216, t91218)
}
