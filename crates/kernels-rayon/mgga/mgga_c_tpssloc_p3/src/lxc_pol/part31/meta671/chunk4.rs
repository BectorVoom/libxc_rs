//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2005/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2005(t84242: f64, t84248: f64, t84280: f64, t91961: f64, t91980: f64, t91996: f64, t92001: f64, t92003: f64, t92008: f64, t92012: f64, t92031: f64, t92034: f64) -> f64 {
    let t102284 = t91961 + t91980 + 176.0_f64 / 27.0_f64 * t91996 - t92001 + 176.0_f64 / 27.0_f64 * t92003 - t92008 - t92012 - 440.0_f64 / 27.0_f64 * t84242 - 176.0_f64 / 27.0_f64 * t84248 - t84280 - t92031 - t92034;
    t102284
}
