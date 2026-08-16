//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 639/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk639(t25: f64, t1799: f64, t571: f64, t3919: f64, t1408: f64, t3664: f64, t2: f64, t514: f64, t584: f64, t606: f64, t1649: f64, t3672: f64, t517: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t5127 = t571 * t1799;
    let t5131 = t3919 * t1799;
    let t5134 = t3664 * t1408;
    let t5137 = t514 * t2;
    let t5141 = piecewise3(t26, 0.0_f64, 4.0_f64 / 9.0_f64 * t5134 * t606 + 8.0_f64 / 3.0_f64 * t5137 * t584);
    let t5142 = t3672 * t1649;
    let t5145 = t517 * t2;
    (t5127, t5131, t5141, t5142, t5145)
}
