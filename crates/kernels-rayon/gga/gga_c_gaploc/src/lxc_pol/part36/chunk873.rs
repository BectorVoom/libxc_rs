//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 873/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk873(t41965: f64, t6717: f64, t6914: f64, t10532: f64, t10533: f64, t42188: f64, t42190: f64, t42194: f64, t42198: f64, t42200: f64, t42203: f64, t42205: f64, t42208: f64, t42210: f64, t42214: f64, t42216: f64, t42221: f64, t42224: f64, t42227: f64, t42230: f64, t42233: f64, t42236: f64, t42239: f64) -> f64 {
    let t42242 = 0.62115540045351614476e2_f64 * t6914 * t6717 * t41965;
    let t42245 = 0.27606906686822939767e2_f64 * t10532 * t10533 * t41965;
    let t42246 = -t42188 + t42190 - t42194 + t42198 - t42200 - t42203 - t42205 - t42208 - 0.21450293971110256002e1_f64 * t42210 - 0.21450293971110256002e1_f64 * t42214 - 0.21450293971110256002e1_f64 * t42216 - t42221 - t42224 - t42227 - t42230 + t42233 + t42236 + t42239 - t42242 + t42245;
    t42246
}
