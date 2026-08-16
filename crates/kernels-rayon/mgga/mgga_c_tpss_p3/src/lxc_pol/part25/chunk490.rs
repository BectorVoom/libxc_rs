//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 490/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk490(t118: f64, t1760: f64, t1796: f64, t1800: f64, t1830: f64, t1834: f64, t1846: f64, t485: f64, t544: f64, t626: f64, t3: f64, param_d: f64) -> (f64, f64, f64) {
    let t1848 = -t118 * t1830 + t1760 * t1846 - t1796 * t485 - 2.0_f64 * t1800 * t626 + t1834 * t544;
    let t1849 = t3 * t1848;
    let t1851 = param_d * t1848;
    (t1848, t1849, t1851)
}
