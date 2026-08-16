//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 543/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk543(t3225: f64, t716: f64, t9664: f64, t9666: f64, t9669: f64, t9672: f64, t9674: f64, t2524: f64, t871: f64, t3228: f64, t471: f64, t64: f64) -> (f64, f64) {
    let t9676 = t3225 * t716;
    let t9678 = -21.0_f64 / 512.0_f64 * t9664 + 147.0_f64 / 16384.0_f64 * t9666 - 63.0_f64 / 1048576.0_f64 * t9669 + 21.0_f64 / 1048576.0_f64 * t9672 - 49.0_f64 / 16384.0_f64 * t9674 + 7.0_f64 / 512.0_f64 * t9676;
    let t9682 = t2524 * t871;
    let t9688 = t9678 * t471 - 4.0_f64 / 3.0_f64 * t3228 * t64 + t9682 / 2.0_f64 - 7.0_f64 / 512.0_f64 * t9664 + 21.0_f64 / 16384.0_f64 * t9666 - 7.0_f64 / 16384.0_f64 * t9674 + 7.0_f64 / 1536.0_f64 * t9676;
    (t9676, t9688)
}
