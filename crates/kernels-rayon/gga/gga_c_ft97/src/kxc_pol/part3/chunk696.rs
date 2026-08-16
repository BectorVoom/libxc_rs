//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 696/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk696(t1882: f64, t3221: f64, t11174: f64, t443: f64, t444: f64, t3283: f64, t103: f64, t7800: f64, t1570: f64, t2266: f64, t1557: f64, t8654: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11999 = 4.0_f64 / 9.0_f64 * t1882 * t3221;
    let t12001 = t443 * t444 * t11174;
    let t12002 = t12001 * t3283;
    let t12020 = t103 * t7800;
    let t12116 = t2266 * t1570;
    let t12122 = t8654 * t1557;
    (t11999, t12001, t12002, t12020, t12116, t12122)
}
