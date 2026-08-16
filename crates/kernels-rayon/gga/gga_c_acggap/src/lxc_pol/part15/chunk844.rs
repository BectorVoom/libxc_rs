//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 844/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk844(t8247: f64, t8249: f64, t8252: f64, t8253: f64, t8254: f64, t8257: f64, t8268: f64, t8269: f64, t8271: f64, t8275: f64, t8276: f64, t8898: f64, t9713: f64, t9715: f64, t9717: f64, t9721: f64, t9725: f64, t9728: f64, t9731: f64, t9735: f64) -> f64 {
    let t9951 = -0.916875e-1_f64 * t9713 - 0.34299214494455789578e-2_f64 * t9715 + 0.34299214494455789578e-2_f64 * t9717 + 0.62896184579208304137e-2_f64 * t9721 - t8247 - t8249 + 0.42874018118069736972e-3_f64 * t8898 + 0.94344276868812456207e-3_f64 * t9725 - t8252 - t8253 + t8254 + t8257 + t9728 / 12.0_f64 + t9731 / 32.0_f64 - 0.4584375e-1_f64 * t9735 - t8268 + t8269 - t8271 + t8275 - t8276;
    t9951
}
