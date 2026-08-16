//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 637/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk637(t1168: f64, t6061: f64, t729: f64, t762: f64, t24658: f64, t24673: f64, t24690: f64, t28236: f64, t28239: f64, t28243: f64, t28248: f64, t28252: f64, t28257: f64, t28260: f64, t28264: f64, t28269: f64, t28273: f64, t446: f64) -> (f64, f64) {
    let t28276 = t6061 * t1168;
    let t28278 = t729 * t762 * t28276;
    let t28281 = t24658 - t24673 / 27.0_f64 + t24690 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t28236 + 2.0_f64 / 3.0_f64 * t446 * t28239 + t446 * t28243 / 3.0_f64 + t446 * t28248 / 3.0_f64 + t446 * t28252 / 3.0_f64 + t446 * t28257 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t28260 + 2.0_f64 / 3.0_f64 * t446 * t28264 + t446 * t28269 / 3.0_f64 + t446 * t28273 / 3.0_f64 + t446 * t28278 / 3.0_f64;
    (t28276, t28281)
}
