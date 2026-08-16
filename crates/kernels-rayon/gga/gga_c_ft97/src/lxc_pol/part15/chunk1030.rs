//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1030/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1030(t37407: f64, t7761: f64, t85469: f64, t89: f64, t28: f64, t74389: f64, t942: f64, t37353: f64, t37356: f64, t356: f64, t359: f64, t85501: f64) -> (f64, f64, f64, f64) {
    let t86246 = t89 * t7761 * t37407 * t85469;
    let t86250 = t89 * t28 * t74389 * t942;
    let t86254 = t89 * t37353 * t37356 * t85469;
    let t86258 = t89 * t356 * t359 * t85501;
    (t86246, t86250, t86254, t86258)
}
