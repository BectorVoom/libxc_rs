//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1322/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1322(t1458: f64, t6514: f64, t1873: f64, t1868: f64, t4072: f64, t33085: f64, t6534: f64, t22461: f64, t7467: f64, t90400: f64, t120112: f64, t114418: f64, t1983: f64, t7687: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t120145 = t6514 * t1458;
    let t120146 = t120145 * t1873;
    let t120148 = t1868 * t4072;
    let t120149 = t120148 * t1873;
    let t120151 = t33085 * t6534;
    let t120153 = t22461 * t7467;
    let t120163 = t90400 * t1873;
    let t120165 = 2.0_f64 * t120112;
    let t120171 = 3.0_f64 * t1983 * t114418 * t7687;
    (t120145, t120146, t120148, t120149, t120151, t120153, t120163, t120165, t120171)
}
