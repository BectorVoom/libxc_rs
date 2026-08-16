//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1121/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1121(t2035: f64, t4088: f64, t7590: f64, t4125: f64, t1701: f64, t28629: f64, t150786: f64, t7607: f64, t153047: f64, t800: f64, t150688: f64, t6243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t153087 = t2035 * t7590 * t4088;
    let t153091 = t2035 * t7590 * t4125;
    let t153094 = t1701 * t28629;
    let t153104 = t7607 * t150786;
    let t153112 = t800 * t153047;
    let t153113 = t150688 * t6243;
    (t153087, t153091, t153094, t153104, t153112, t153113)
}
