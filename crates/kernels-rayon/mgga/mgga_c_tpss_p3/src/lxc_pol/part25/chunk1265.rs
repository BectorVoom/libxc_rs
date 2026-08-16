//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1265/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1265(t1339: f64, t1663: f64, t1760: f64, t1796: f64, t1800: f64, t1830: f64, t18547: f64, t20289: f64, t21236: f64, t21880: f64, t21883: f64, t21894: f64, t21897: f64, t21900: f64, t21908: f64, t21922: f64, t21944: f64, t3493: f64, t4638: f64, t485: f64, t5314: f64, t544: f64, t6103: f64, t6243: f64, t626: f64, t6318: f64, t6324: f64, t6328: f64, t6409: f64, t6413: f64, t6439: f64) -> f64 {
    let t21946 = -4.0_f64 * t1339 * t20289 + 2.0_f64 * t1663 * t6409 + 3.0_f64 * t1760 * t21883 - t1796 * t5314 - 2.0_f64 * t1800 * t21236 - 2.0_f64 * t1830 * t4638 - 6.0_f64 * t18547 * t21900 - 4.0_f64 * t21880 * t626 - 2.0_f64 * t21894 * t626 - 2.0_f64 * t21897 * t626 - 2.0_f64 * t21908 * t626 - 2.0_f64 * t21922 * t485 + t21944 * t544 - 4.0_f64 * t3493 * t6318 - 4.0_f64 * t3493 * t6324 - 4.0_f64 * t3493 * t6328 - 4.0_f64 * t6103 * t6318 - 4.0_f64 * t6103 * t6324 + 6.0_f64 * t6243 * t6413 - 2.0_f64 * t6243 * t6439;
    t21946
}
