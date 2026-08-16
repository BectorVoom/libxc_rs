//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 683/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk683(t3450: f64, t5942: f64, t12968: f64, t23455: f64, t3455: f64, t13140: f64, t6695: f64, t9099: f64, t379: f64, t6639: f64, t9144: f64, t574: f64, t5935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26924 = t5942 * t3450;
    let t26925 = t12968 * t26924;
    let t26928 = t23455 * t3455;
    let t26929 = t13140 * t26928;
    let t26932 = t9099 * t6695;
    let t26935 = t6639 * t379;
    let t26936 = t9144 * t26935;
    let t26940 = t574 * t5935 * t3455;
    (t26924, t26925, t26928, t26929, t26932, t26935, t26936, t26940)
}
