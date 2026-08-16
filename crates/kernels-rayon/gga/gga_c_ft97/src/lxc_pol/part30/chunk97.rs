//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 97/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk97(t231: f64, t240: f64, t247: f64, t342: f64, t343: f64, t10: f64, t242: f64, t351: f64, t322: f64) -> (f64, f64, f64, f64, f64) {
    let t657 = t231 * t240;
    let t661 = t247 - t342 * t343 * t657 / 4.0_f64;
    let t663 = t10 * t351 * t242;
    let t664 = t663 / 18.0_f64;
    let t665 = 1.0_f64 / t322;
    (t657, t661, t663, t664, t665)
}
