//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 827/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk827(t109: f64, t656: f64, t9411: f64, t64: f64, t9358: f64, t9359: f64, t9361: f64, t9363: f64, t9367: f64, t9371: f64) -> f64 {
    let t110 = 1.0_f64 < t109;
    let t9412 = t656 * t9411;
    let t9416 = piecewise3(t110, 0.0_f64, -t9358 - 11.0_f64 / 3.0_f64 * t9359 - 2.0_f64 * t9361 + t9363 - 3.0_f64 / 4.0_f64 * t64 * t9367 + 3.0_f64 / 4.0_f64 * t64 * t9371 - t64 * t9412 / 8.0_f64);
    t9416
}
