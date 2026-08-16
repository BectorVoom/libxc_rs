//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1211/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1211(t40546: f64, t40564: f64, t42452: f64, t42455: f64, t42456: f64, t42457: f64, t42458: f64, t42460: f64, t42461: f64, t48217: f64, t48221: f64, t48225: f64, t48231: f64) -> f64 {
    let t48233 = 0.38342925953920749677e0_f64 * t40546;
    let t48235 = -0.61348681526273199483e1_f64 * t48217 - 0.46011511144704899612e1_f64 * t48221 - 0.46011511144704899612e1_f64 * t48225 - t48231 - 0.25025342966295298669e1_f64 * t42452 + t48233 + t42455 - t42456 + t42457 - t42458 + 0.10224780254378866581e1_f64 * t40564 - t42460 + t42461;
    t48235
}
