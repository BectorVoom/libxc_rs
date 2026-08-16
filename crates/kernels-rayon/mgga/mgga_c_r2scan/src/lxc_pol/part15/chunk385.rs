//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 385/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk385(t1542: f64, t1543: f64, t1234: f64, t490: f64, t109: f64, t111: f64, t1536: f64, t486: f64, t491: f64) -> (f64, f64, f64) {
    let t1544 = t1542 * t1543;
    let t1547 = t490 * t1234;
    let t1550 = -12.0_f64 * t109 * t1544 + 3.0_f64 * t109 * t1547 - t1536 * t111 + 6.0_f64 * t486 * t491;
    (t1544, t1547, t1550)
}
