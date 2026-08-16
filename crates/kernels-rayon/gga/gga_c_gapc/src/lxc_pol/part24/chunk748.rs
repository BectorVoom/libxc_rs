//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 748/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk748(t102: f64, t8894: f64, t1648: f64, t1894: f64, t8893: f64, t1026: f64, t1846: f64, t637: f64, t1510: f64, t2982: f64, t3084: f64, t3131: f64, t3707: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8895 = t8894 * t102;
    let t8897 = t8895 * t1648 * t1894;
    let t8898 = t8893 * t8897;
    let t8900 = t1846 * t1026;
    let t8901 = t8900 * t637;
    let t8903 = t2982 * t1510;
    let t8904 = t3084 * t8903;
    let t8906 = t3131 * t3707;
    (t8895, t8898, t8901, t8903, t8904, t8906)
}
