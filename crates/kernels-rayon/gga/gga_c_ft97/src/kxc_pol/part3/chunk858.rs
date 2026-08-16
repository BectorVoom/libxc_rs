//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 858/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk858(t17295: f64, t17346: f64, t579: f64, t91: f64, t16925: f64, t16928: f64, t16739: f64, t16742: f64, t16756: f64, t16760: f64, t16922: f64, t17249: f64, t17250: f64, t17251: f64) -> (f64, f64) {
    let t17347 = t17295 + t17346;
    let t17349 = t91 * t579 * t17347;
    let t17351 = t16925 / 3.0_f64;
    let t17352 = 2.0_f64 / 3.0_f64 * t16928;
    let t17353 = -6.0_f64 * t16739 + 4.0_f64 * t16742 + t17249 - t17250 + t17251 + 2.0_f64 * t16756 - t16760 / 3.0_f64 + t17349 / 2.0_f64 - t16922 + t17351 - t17352;
    (t17349, t17353)
}
