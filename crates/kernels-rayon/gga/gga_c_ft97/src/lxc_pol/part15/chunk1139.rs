//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1139/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1139(t2345: f64, t2348: f64, t88239: f64, t89: f64, t2361: f64, t666: f64, t42759: f64, t80819: f64, t80821: f64, t88218: f64, t88221: f64, t88225: f64, t88229: f64, t88233: f64, t88237: f64, t89022: f64, t89027: f64, t89030: f64, t89034: f64) -> (f64, f64, f64) {
    let t89038 = t89 * t2345 * t2348 * t88239;
    let t89042 = t89 * t666 * t2361 * t88239;
    let t89044 = -t88218 + t42759 + 4.0_f64 / 3.0_f64 * t88221 - 4.0_f64 * t88225 - 8.0_f64 / 3.0_f64 * t88229 - 3.0_f64 / 4.0_f64 * t88233 - 15.0_f64 / 16.0_f64 * t88237 + t89022 / 2.0_f64 - 4.0_f64 / 3.0_f64 * t80819 - 4.0_f64 / 3.0_f64 * t80821 - 20.0_f64 / 9.0_f64 * t89027 - 12.0_f64 * t89030 - 8.0_f64 * t89034 - 2.0_f64 / 3.0_f64 * t89038 + 2.0_f64 * t89042;
    (t89038, t89042, t89044)
}
