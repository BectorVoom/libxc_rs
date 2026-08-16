//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 873/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk873(t605: f64, t9114: f64, t142: f64, t7763: f64, t342: f64, t511: f64, t8639: f64, t7800: f64, t10050: f64, t257: f64, t255: f64, t2346: f64, t2359: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41269 = t9114 * t605;
    let t41318 = t142 * t7763;
    let t41328 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t511;
    let t41349 = t142 * t7800;
    let t41408 = 1.0_f64 / t10050 / t257;
    let t41409 = t255 * t41408;
    let t41446 = 1.0_f64 / t2346 / t2359;
    (t41269, t41318, t41328, t41349, t41409, t41446)
}
