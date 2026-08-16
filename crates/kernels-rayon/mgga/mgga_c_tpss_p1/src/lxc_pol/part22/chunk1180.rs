//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1180/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1180(t12669: f64, t13251: f64, t3: f64, t1338: f64, t2061: f64, t116: f64, t3537: f64, t645: f64, t2105: f64, t4555: f64, t117: f64, t13220: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13252 = t12669 + t13251;
    let t13253 = t3 * t13252;
    let t13265 = param_d * t13252;
    let t13279 = t2061 * t1338;
    let t13282 = t116 * t3537;
    let t13283 = t13282 * t645;
    let t13286 = t4555 * t2105;
    let t13289 = t117 * t13220;
    (t13253, t13265, t13279, t13283, t13286, t13289)
}
