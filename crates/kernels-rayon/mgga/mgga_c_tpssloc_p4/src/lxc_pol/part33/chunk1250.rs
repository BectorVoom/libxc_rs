//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1250/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1250(t23030: f64, t25205: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64, t10143: f64, t7540: f64, t1625: f64, t225: f64, t344: f64, t3173: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t87898 = t23030 * t25205;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t87975 = t7540 * t10143;
    let t88050 = t344 * t1625 * t225;
    let t88076 = t3173 * t883;
    (t87898, t87915, t87975, t88050, t88076)
}
