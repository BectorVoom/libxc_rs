//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1474/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1474(t1484: f64, t828: f64, t2647: f64, t13350: f64, t1516: f64, t9993: f64, t2696: f64, t4166: f64, t849: f64, t13176: f64, t842: f64, t9601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13351 = t1484 * t828;
    let t13352 = t13351 * t2647;
    let t13353 = t13350 * t13352;
    let t13359 = 7.0_f64 / 576.0_f64 * t9993 * t1516;
    let t13360 = t4166 * t2696;
    let t13362 = 7.0_f64 / 576.0_f64 * t13360 * t849;
    let t13365 = t13176 * t842;
    let t13368 = t9601 * t1516;
    (t13351, t13352, t13353, t13359, t13362, t13365, t13368)
}
