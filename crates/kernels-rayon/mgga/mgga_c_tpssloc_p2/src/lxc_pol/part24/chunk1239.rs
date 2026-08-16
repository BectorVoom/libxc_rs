//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1239/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1239(t23862: f64, t580: f64, t23901: f64, t576: f64, t1404: f64, t7002: f64, t2029: f64, t3931: f64, t2022: f64, t3946: f64, t1983: f64, t23857: f64, t6996: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80593 = t23862 * t580;
    let t80597 = t576 * t23901;
    let t80599 = t7002 * t1404;
    let t80601 = t3931 * t2029;
    let t80605 = t2022 * t3946;
    let t80609 = 6.0_f64 * t1983 * t6996 * t23857;
    (t80593, t80597, t80599, t80601, t80605, t80609)
}
