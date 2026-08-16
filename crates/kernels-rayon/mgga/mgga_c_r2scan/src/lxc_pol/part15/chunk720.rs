//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 720/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk720(t225: f64, t5429: f64, t1719: f64, t1986: f64, t5317: f64, t721: f64, t1647: f64, t645: f64, t650: f64, t648: f64, t14: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5658 = t5429 * t225;
    let t5661 = t1986 * t1719;
    let t5664 = t721 * t5317;
    let t5669 = 18.0_f64 * t650 * t645 * t1647;
    let t5670 = t648 * t648;
    let t5671 = 1.0_f64 / t5670;
    let t5672 = t14 * t5671;
    let t5673 = t651 * t651;
    (t5658, t5661, t5664, t5669, t5672, t5673)
}
