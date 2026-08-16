//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 412/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk412(t1669: f64, t612: f64, t585: f64, t607: f64, t159: f64, t617: f64) -> (f64, f64, f64, f64, f64) {
    let t1671 = 0.11290853155555555555e-2_f64 * t612 * t1669;
    let t1672 = t607 * t585;
    let t1673 = t159 * t1672;
    let t1674 = t1673 * t617;
    let t1676 = t585 * t585;
    let t1677 = t1676 * t1676;
    let t1678 = t1677 * t585;
    (t1671, t1672, t1673, t1674, t1678)
}
