//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1227/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1227(t23602: f64, t25490: f64, t1936: f64, t362: f64, t2775: f64, t381: f64, t23509: f64, t3: f64, t23470: f64, t3030: f64, t1022: f64, t23678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25491 = t23602 * t25490;
    let t25510 = t1936 * t362;
    let t25511 = t381 * t2775;
    let t25650 = t23509 * t3;
    let t25651 = t23470 * t3030;
    let t25652 = t25650 * t25651;
    let t25654 = t23678 * t1022;
    (t25491, t25510, t25511, t25650, t25651, t25652, t25654)
}
