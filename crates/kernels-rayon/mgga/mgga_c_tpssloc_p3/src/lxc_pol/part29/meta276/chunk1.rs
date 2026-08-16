//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1281/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1281(t5: f64, t1860: f64, t2110: f64, t7246: f64, t7428: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64, t112: f64, t1458: f64, t2165: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7982 = piecewise3(t8, 0.0_f64, -t7428 * t2110 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t7246 * t7432 + t7435 * t2110 / 3.0_f64 - t1860 * t7975 / 6.0_f64 - t1860 * t7978 / 6.0_f64);
    let t7983 = t7982 * t112;
    let t7989 = t2165 * t1458;
    (t7982, t7983, t7989)
}
