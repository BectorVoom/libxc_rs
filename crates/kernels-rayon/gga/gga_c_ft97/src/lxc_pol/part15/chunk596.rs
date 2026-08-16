//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 596/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk596(t2360: f64, t294: f64, t2680: f64, t683: f64, t191: f64, t7640: f64, t295: f64, t9570: f64, t272: f64, t9606: f64, t274: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10222 = t294 * t2360;
    let t10248 = t683 * t2680;
    let t10261 = t191 * t7640;
    let t10270 = t295 * t9570;
    let t10304 = 1.0_f64 / t272 / t9606;
    let t10327 = t274 * t668;
    (t10222, t10248, t10261, t10270, t10304, t10327)
}
