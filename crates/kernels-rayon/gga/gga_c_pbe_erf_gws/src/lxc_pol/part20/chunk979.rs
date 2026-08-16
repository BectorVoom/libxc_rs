//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 979/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk979(t11048: f64, t639: f64, t10539: f64, t7210: f64, t3390: f64, t626: f64, t422: f64, t4927: f64, t3473: f64, t617: f64, t1809: f64, t1620: f64) -> (f64, f64, f64, f64) {
    let t11050 = 32.0_f64 / 81.0_f64 * t639 * t11048;
    let t11051 = t7210 * t10539;
    let t11053 = 16.0_f64 / 27.0_f64 * t639 * t11051;
    let t11054 = t3390 * t626;
    let t11055 = t11054 * t422;
    let t11056 = t4927 * t11055;
    let t11058 = 8.0_f64 / 45.0_f64 * t639 * t11056;
    let t11059 = t3473 * t617;
    let t11060 = t1809 * t11059;
    let t11062 = 8.0_f64 / 45.0_f64 * t1620 * t11060;
    (t11050, t11053, t11058, t11062)
}
