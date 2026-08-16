//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 65/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk65(t255: f64, t260: f64, t109: f64, t111: f64) -> (f64, f64, f64) {
    let t271 = 1.0_f64 / t255;
    let t275 = t260 * t260;
    let t277 = 0.50765919958333333334e-3_f64 * t109 * t111 * t271 - 2.0_f64 * t275;
    let t278 = 1.0_f64 / t277;
    (t271, t277, t278)
}
