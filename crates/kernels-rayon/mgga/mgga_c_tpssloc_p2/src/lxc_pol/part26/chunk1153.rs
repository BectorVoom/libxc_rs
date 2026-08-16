//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1153/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1153(t16535: f64, t1873: f64, t6534: f64, t671: f64, t3941: f64, t2363: f64, t1401: f64, t22479: f64, t2274: f64, t50: f64, t2244: f64, t2250: f64, t22510: f64, t7251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23892 = 27.0_f64 * t16535 * t1873;
    let t23893 = t6534 * t671;
    let t23895 = 54.0_f64 * t3941 * t23893;
    let t23896 = t1873 * t2363;
    let t23898 = 27.0_f64 * t3941 * t23896;
    let t23900 = 0.135e2_f64 * t1401 * t22479;
    let t24498 = t50 * t2274;
    let t24503 = 5.0_f64 / 18.0_f64 * t24498 * t2244 - 5.0_f64 / 6.0_f64 * t7251 * t2250 - t22510;
    (t23892, t23893, t23895, t23896, t23898, t23900, t24498, t24503)
}
