//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1840/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1840(t13931: f64, t340: f64, t343: f64, t974: f64, t10263: f64, t10287: f64, t10290: f64, t10331: f64, t10333: f64, t10339: f64, t10342: f64, t10353: f64, t13896: f64, t13907: f64, t13909: f64, t13915: f64, t1600: f64, t2960: f64, t4543: f64, t973: f64) -> (f64, f64) {
    let t13933 = t340 * t13931 * t343;
    let t13934 = t974 * t13933;
    let t13937 = -0.6172839506172839506e-4_f64 * t13896 + 0.37037037037037037036e-3_f64 * t10287 - 0.27777777777777777777e-3_f64 * t10290 + 0.27160493827160493826e-2_f64 * t10331 + 0.98765432098765432096e-3_f64 * t10333 + t10339 + 0.14814814814814814814e-2_f64 * t10342 - 0.27777777777777777777e-3_f64 * t10353 - 0.81481481481481481481e-2_f64 * t10263 * t1600 + t13907 + 0.18518518518518518518e-3_f64 * t13909 + 0.44444444444444444444e-2_f64 * t2960 * t4543 - t13915 - 0.83333333333333333332e-3_f64 * t973 * t13934;
    (t13933, t13937)
}
