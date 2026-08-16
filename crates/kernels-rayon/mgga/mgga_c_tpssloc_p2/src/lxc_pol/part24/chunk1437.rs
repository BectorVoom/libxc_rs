//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1437/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1437(t1395: f64, t7020: f64, t12513: f64, t12537: f64, t1396: f64, t1398: f64, t1404: f64, t2023: f64, t2029: f64, t23863: f64, t23901: f64, t3: f64, t3932: f64, t3946: f64, t580: f64, t7003: f64, t80593: f64, t80597: f64, t80599: f64, t80601: f64, t80605: f64, t83973: f64, t84019: f64) -> f64 {
    let t84024 = t1395 * t7020;
    let tv4rho3sigma0 = t3 * t580 * t83973 + t12513 * t2029 + t12537 * t2023 + 3.0_f64 * t1396 * t23901 + t1398 * t84019 + 3.0_f64 * t1404 * t23863 + 3.0_f64 * t3932 * t7020 + 3.0_f64 * t3946 * t7003 + 3.0_f64 * t80593 + 3.0_f64 * t80597 + 6.0_f64 * t80599 + 3.0_f64 * t80601 + 3.0_f64 * t80605 + 6.0_f64 * t84024;
    tv4rho3sigma0
}
