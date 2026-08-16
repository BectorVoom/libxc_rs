//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 721/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk721(t1882: f64, t3866: f64, t3844: f64, t255: f64, t9952: f64, t258: f64, t9570: f64, t9577: f64, t1162: f64, t2399: f64, t89: f64, t3871: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14020 = 4.0_f64 / 9.0_f64 * t1882 * t3866;
    let t14052 = 2.0_f64 / 9.0_f64 * t1882 * t3844;
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14098 = t258 * t9577;
    let t14114 = t89 * t2399 * t1162;
    let t14126 = 2.0_f64 / 27.0_f64 * t8392 * t3871;
    (t14020, t14052, t14080, t14081, t14098, t14114, t14126)
}
