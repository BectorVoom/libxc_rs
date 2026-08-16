//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1163/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1163(t1136: f64, t21122: f64, t258: f64, t263: f64, t5058: f64, t5059: f64, t5179: f64, t661: f64, t88131: f64, t89092: f64, t89097: f64, t89179: f64, t89442: f64, t89465: f64, t89547: f64, t89565: f64, t89685: f64, t89704: f64, t89712: f64, t89727: f64, t89741: f64) -> f64 {
    let t89749 = -3.0_f64 * t21122 * t5058 * t263 - 8.0_f64 * t89565 - 12.0_f64 * t88131 + 48.0_f64 * t89442 - 72.0_f64 * t89465 - t89685 * t1136 * t263 - 8.0_f64 * t89179 + 12.0_f64 * t89097 + 2.0_f64 * t89547 * t258 - t661 * (t89704 + t89712 + t89727 + t89741) * t263 - 2.0_f64 * t89092 - 6.0_f64 * t5059 * t5179;
    t89749
}
