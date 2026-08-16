//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2103/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2103(t26555: f64, t576: f64, t1858: f64, t7002: f64, t2029: f64, t5363: f64, t1851: f64, t7020: f64, t16507: f64, t16546: f64, t1852: f64, t2023: f64, t23863: f64, t23901: f64, t3946: f64, t5381: f64, t7003: f64, t7759: f64, t80593: f64, t80597: f64, t84024: f64) -> f64 {
    let t86565 = 2.0_f64 * t576 * t26555;
    let t86567 = 2.0_f64 * t7002 * t1858;
    let t86571 = 2.0_f64 * t5363 * t2029;
    let t86579 = 2.0_f64 * t1851 * t7020;
    let t86580 = t16507 * t2029 + t16546 * t2023 + t1852 * t23901 + t1858 * t23863 + t3946 * t7759 + 2.0_f64 * t5381 * t7003 + t80593 + t80597 + 2.0_f64 * t84024 + t86565 + t86567 + t86571 + t86579;
    t86580
}
