//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 414/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk414(t240: f64, t668: f64, t505: f64, t231: f64, t713: f64, t1526: f64, t2319: f64, t2320: f64, t342: f64, t343: f64, t719: f64, t718: f64) -> (f64, f64, f64, f64, f64) {
    let t2321 = t240 * t668;
    let t2322 = t2321 * t505;
    let t2326 = t231 * t713;
    let t2330 = t719 - t2319 - t1526 * t2320 * t2322 / 12.0_f64 - t342 * t343 * t2326 / 4.0_f64;
    let t2331 = t2330 * t718;
    (t2321, t2322, t2326, t2330, t2331)
}
