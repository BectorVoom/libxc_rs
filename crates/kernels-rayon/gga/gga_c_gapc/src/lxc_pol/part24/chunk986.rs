//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 986/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk986(t11428: f64, t291: f64, t1461: f64, t1084: f64, t2763: f64, t332: f64, t10078: f64, t6: f64, t11597: f64, t3415: f64, t644: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11922 = t11428 * t291;
    let t11923 = t1461 * t11922;
    let t11924 = t1084 * t11923;
    let t11925 = t2763 * t332;
    let t11927 = t11925 * t6 * t10078;
    let t11928 = t11924 * t11927;
    let t11930 = t1084 * t11597;
    let t11931 = t11930 * t3415;
    let t11933 = t825 * t644;
    (t11923, t11924, t11925, t11927, t11928, t11930, t11931, t11933)
}
