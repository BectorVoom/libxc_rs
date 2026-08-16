//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 989/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk989(t19034: f64, t19306: f64, t788: f64, t1882: f64, t5332: f64, t5323: f64, t5319: f64, t1212: f64, t4299: f64, t840: f64, t871: f64, t4246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19307 = t19034 + t19306;
    let t19308 = t788 * t19307;
    let t19318 = t1882 * t5332;
    let t19320 = t1882 * t5323;
    let t19322 = t1882 * t5319;
    let t19324 = t1212 * t4299;
    let t19326 = t840 * t871 * t19324;
    let t19329 = t4246 * t4299;
    (t19308, t19318, t19320, t19322, t19326, t19329)
}
