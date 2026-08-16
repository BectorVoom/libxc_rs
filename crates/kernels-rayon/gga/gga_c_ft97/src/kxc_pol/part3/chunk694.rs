//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 694/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk694(t1852: f64, t463: f64, t110: f64, t8216: f64, t1882: f64, t3210: f64, t8232: f64, t951: f64, t3216: f64, t1786: f64, t971: f64, t3184: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11854 = t463 * t1852;
    let t11863 = t8216 * t110;
    let t11882 = 2.0_f64 / 27.0_f64 * t1882 * t3210;
    let t11883 = t8232 * t951;
    let t11897 = 2.0_f64 / 9.0_f64 * t1882 * t3216;
    let t11902 = t1786 * t971;
    let t11906 = t463 * t971;
    let t11913 = 2.0_f64 / 27.0_f64 * t8392 * t3184;
    (t11854, t11863, t11882, t11883, t11897, t11902, t11906, t11913)
}
