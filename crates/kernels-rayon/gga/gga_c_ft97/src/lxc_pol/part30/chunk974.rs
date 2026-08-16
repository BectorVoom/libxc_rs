//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 974/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk974(t2360: f64, t7611: f64, t2680: f64, t33953: f64, t33967: f64, t681: f64, t89: f64, t33971: f64, t33984: f64, t1882: f64, t33963: f64, t1486: f64, t2399: f64, t7646: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t143284 = t7611 * t2360;
    let t143293 = t2680 * t33953;
    let t143321 = t89 * t681 * t33967;
    let t143324 = t89 * t681 * t33971;
    let t143327 = t89 * t681 * t33984;
    let t143329 = t1882 * t33963;
    let t143332 = t1486 * t2399 * t7646;
    (t143284, t143293, t143321, t143324, t143327, t143329, t143332)
}
