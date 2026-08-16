//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 399/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk399(t1147: f64, t1155: f64, t1156: f64, t1164: f64, t134: f64, t457: f64, t461: f64, t221: f64, t456: f64, t51: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1166 = t1147 * t1155 * t1156;
    let t1168 = 0.5848223622634646207e0_f64 * t1164 * t1166;
    let t1169 = t134 * t457;
    let t1170 = t1169 * t461;
    let t1171 = t221 * t1170;
    let t1173 = 0.27777777777777777777e-3_f64 * t456 * t1171;
    let t1174 = t51 * t972;
    (t1166, t1168, t1169, t1171, t1173, t1174)
}
