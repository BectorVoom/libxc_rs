//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 759/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk759(t1748: f64, t322: f64, t1165: f64, t1532: f64, t3194: f64, t301: f64, t513: f64, t944: f64, t406: f64, t1552: f64, t495: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5730 = t1748 * t322;
    let t5732 = t1165 * t1532 * t5730;
    let t5733 = t3194 * t5732;
    let t5735 = t1748 * t301;
    let t5737 = t1165 * t1532 * t5735;
    let t5740 = t944 * t513;
    let t5741 = t5740 * t406;
    let t5743 = t1165 * t1552 * t5741;
    let t5746 = t944 * t495;
    let t5747 = t5746 * t406;
    let t5749 = t1165 * t1532 * t5747;
    let t5752 = t944 * t506;
    (t5730, t5732, t5733, t5735, t5737, t5740, t5741, t5743, t5746, t5747, t5749, t5752)
}
