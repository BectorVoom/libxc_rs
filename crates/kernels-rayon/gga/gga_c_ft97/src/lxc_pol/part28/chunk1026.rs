//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1026/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1026(t144914: f64, t1564: f64, t363: f64, t446: f64, t1882: f64, t34488: f64, t18: f64, t32333: f64, t3281: f64, t7824: f64, t1317: f64, t34483: f64, t376: f64) -> (f64, f64, f64, f64) {
    let t144917 = t446 * t1564 * t144914 * t363;
    let t144919 = t1882 * t34488;
    let t144923 = t3281 * t7824 * t32333 * t18;
    let t144926 = t1317 * t376 * t34483;
    (t144917, t144919, t144923, t144926)
}
