//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 787/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk787(t10286: f64, t10243: f64, t2755: f64, t2789: f64, t856: f64, t91: f64, t10397: f64, t10251: f64, t10255: f64, t10404: f64, t10407: f64, t10412: f64, t10417: f64, t10420: f64, t10424: f64, t10428: f64) -> (f64, f64) {
    let t10643 = 2.0_f64 / 27.0_f64 * t10286;
    let t10649 = 2.0_f64 / 9.0_f64 * t10243;
    let t10656 = t91 * t2755 * t856 * t2789;
    let t10658 = 28.0_f64 / 81.0_f64 * t10397;
    let t10659 = t10643 + 2.0_f64 / 3.0_f64 * t10407 + 2.0_f64 / 9.0_f64 * t10412 - 2.0_f64 / 9.0_f64 * t10420 + t10424 / 3.0_f64 + t10428 / 3.0_f64 - t10649 - 2.0_f64 / 3.0_f64 * t10251 - 2.0_f64 / 3.0_f64 * t10255 - 2.0_f64 / 3.0_f64 * t10404 + 4.0_f64 / 9.0_f64 * t10417 - t10656 / 4.0_f64 - t10658;
    (t10656, t10659)
}
