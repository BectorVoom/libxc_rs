//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 583/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk583(t143: f64, t160: f64, t4790: f64, t2149: f64, t3318: f64, t3335: f64, t4654: f64, t4658: f64, t4662: f64, t4666: f64, t4671: f64, t4717: f64, t4755: f64, t4780: f64) -> (f64, f64) {
    let t4792 = t143 * t4790 * t160;
    let t4805 = -t4755 / 4.0_f64 + t4780 / 2.0_f64 + t2149 + 2.0_f64 / 9.0_f64 * t3318 + 2.0_f64 / 3.0_f64 * t3335 - 2.0_f64 / 9.0_f64 * t4654 + 2.0_f64 / 3.0_f64 * t4658 + 2.0_f64 / 3.0_f64 * t4662 - t4666 / 3.0_f64 + 2.0_f64 * t4671 - t4717;
    (t4792, t4805)
}
