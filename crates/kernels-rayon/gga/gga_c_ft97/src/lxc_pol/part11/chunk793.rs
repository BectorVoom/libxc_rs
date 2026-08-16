//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 793/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk793(t8232: f64, t877: f64, t2739: f64, t840: f64, t882: f64, t10388: f64, t319: f64, t2834: f64, t681: f64, t89: f64, t313: f64, t9555: f64) -> (f64, f64, f64, f64, f64) {
    let t10735 = t8232 * t877;
    let t10738 = t840 * t882 * t2739;
    let t10741 = t840 * t319 * t10388;
    let t10745 = t89 * t681 * t2834;
    let t10749 = 28.0_f64 / 81.0_f64 * t89 * t9555 * t313;
    (t10735, t10738, t10741, t10745, t10749)
}
