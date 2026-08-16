//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 962/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk962(t1971: f64, t3351: f64, t77393: f64, t875: f64, t15523: f64, t7720: f64, t1550: f64, t2069: f64, t2471: f64, t2074: f64, t903: f64, t75006: f64) -> (f64, f64, f64, f64, f64) {
    let t77396 = t3351 * t1971 * t875 * t77393;
    let t77397 = 0.85129199786595678796e-5_f64 * t77396;
    let t77398 = t7720 * t15523;
    let t77399 = 0.42564599893297839398e-5_f64 * t77398;
    let t77401 = t1550 * t2471 * t2069;
    let t77402 = 0.2993560425465952141e-1_f64 * t77401;
    let t77404 = t903 * t2471 * t2074;
    let t77405 = 0.44903406381989282115e-1_f64 * t77404;
    let t77406 = 0.85129199786595678799e-5_f64 * t75006;
    (t77397, t77399, t77402, t77405, t77406)
}
