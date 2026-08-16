//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1380/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1380(t2043: f64, t5999: f64, t1466: f64, t7380: f64, t1535: f64, t1552: f64, t7322: f64, t1543: f64, t7287: f64, t17474: f64, t5932: f64, t7332: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22697 = t5999 * t2043;
    let t22699 = t7380 * t1466;
    let t22700 = t22699 * sigma2;
    let t22701 = t22700 * t1535;
    let t22703 = t7322 * t1552;
    let t22705 = t1543 * t7287;
    let t22707 = t17474 * t5932;
    let t22709 = t1543 * t7332;
    (t22697, t22701, t22703, t22705, t22707, t22709)
}
