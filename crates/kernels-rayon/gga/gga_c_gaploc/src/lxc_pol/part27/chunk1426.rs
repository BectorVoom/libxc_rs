//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1426/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1426(t12236: f64, t1402: f64, t2033: f64, t28406: f64, t28407: f64, t28409: f64, t28415: f64, t28419: f64, t28421: f64, t33130: f64, t33132: f64, t33134: f64, t33136: f64, t33145: f64, t33151: f64, t33154: f64, t33158: f64, t33164: f64) -> f64 {
    let t39039 = -t33130 - t33132 + t33134 + t33136 - t33145 - 0.92686455430723328401e-1_f64 * t2033 * t1402 * t12236 + t33151 - t33154 - t28406 - 0.51123901271894332903e1_f64 * t28407 + 0.30674340763136599742e1_f64 * t28409 - t28415 + t33158 - t33164 - t28419 - t28421;
    t39039
}
