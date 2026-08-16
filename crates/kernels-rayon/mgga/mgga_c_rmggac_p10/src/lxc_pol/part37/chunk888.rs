//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 888/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk888(t11599: f64, t498: f64, t14236: f64, t14237: f64, t2078: f64, t11662: f64, t14243: f64, t11666: f64, t14249: f64, t1971: f64, t2144: f64, t495: f64, t7230: f64, t8946: f64) -> (f64, f64, f64, f64) {
    let t75925 = t11599 * t498;
    let t75928 = t14236 * t14237 * t2078 * t75925;
    let t75932 = t14236 * t14243 * t2078 * t11662;
    let t75936 = t14236 * t14249 * t2078 * t11666;
    let t75943 = t7230 * t1971 * t2144 * t8946 * t495;
    (t75928, t75932, t75936, t75943)
}
