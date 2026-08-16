//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 755/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk755(t109: f64, t16535: f64, t1873: f64, t6534: f64, t671: f64, t3941: f64, t2363: f64, t1401: f64, t22479: f64, t2039: f64, t3652: f64, t22468: f64, t22471: f64, t22474: f64, t22476: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t23892 = 27.0_f64 * t16535 * t1873;
    let t23893 = t6534 * t671;
    let t23895 = 54.0_f64 * t3941 * t23893;
    let t23896 = t1873 * t2363;
    let t23898 = 27.0_f64 * t3941 * t23896;
    let t23900 = 0.135e2_f64 * t1401 * t22479;
    let t23909 = t3652 * t2039;
    let t23912 = 22.0_f64 / 9.0_f64 * t22468;
    let t23917 = piecewise3(t110, 0.0_f64, t23912 + 4.0_f64 / 3.0_f64 * t22471 + t22474 / 2.0_f64 - t22476 / 4.0_f64);
    (t23892, t23893, t23895, t23896, t23898, t23900, t23909, t23917)
}
