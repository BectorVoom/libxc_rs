//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 835/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk835(t14011: f64, t14047: f64, t75027: f64, t11654: f64, t14236: f64, t14249: f64, t2078: f64, t2812: f64, t880: f64, t1971: f64, t14258: f64, t14116: f64, t14117: f64, t8446: f64) -> (f64, f64, f64, f64) {
    let t75029 = t14047 * t14011 * t75027;
    let t75033 = t14236 * t14249 * t2078 * t11654;
    let t75035 = t880 * t2812;
    let t75036 = t1971 * t75035;
    let t75037 = t14258 * t75036;
    let t75040 = t14116 * t14117 * t8446;
    (t75029, t75033, t75037, t75040)
}
