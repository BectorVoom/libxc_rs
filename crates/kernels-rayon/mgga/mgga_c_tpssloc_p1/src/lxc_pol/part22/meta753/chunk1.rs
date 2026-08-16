//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2530/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2530(t71225: f64, t71227: f64, t71230: f64, t71233: f64, t71236: f64, t71238: f64, t71241: f64, t71245: f64, t71247: f64, t71249: f64, t71251: f64, t18934: f64, t4869: f64) -> (f64, f64) {
    let t71252 = -t71225 + t71227 + t71230 - t71233 - t71236 + t71238 - t71241 + t71245 - t71247 - t71249 - t71251;
    let t71255 = 0.35089341735807877242e1_f64 * t4869 * t18934;
    (t71252, t71255)
}
