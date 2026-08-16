//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 574/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk574(t9664: f64, t9666: f64, t9669: f64, t9672: f64, t9674: f64, t9676: f64, t471: f64, t3427: f64, t64: f64, t2919: f64, t871: f64) -> (f64, f64, f64) {
    let t10657 = -21.0_f64 / 256.0_f64 * t9664 + 147.0_f64 / 8192.0_f64 * t9666 - 63.0_f64 / 524288.0_f64 * t9669 + 21.0_f64 / 524288.0_f64 * t9672 - 49.0_f64 / 8192.0_f64 * t9674 + 7.0_f64 / 256.0_f64 * t9676;
    let t10658 = t10657 * t471;
    let t10660 = 4.0_f64 / 3.0_f64 * t3427 * t64;
    let t10661 = t2919 * t871;
    let t10663 = 7.0_f64 / 256.0_f64 * t9664;
    let t10664 = 21.0_f64 / 8192.0_f64 * t9666;
    let t10665 = 7.0_f64 / 8192.0_f64 * t9674;
    let t10666 = 7.0_f64 / 768.0_f64 * t9676;
    let t10667 = t10658 - t10660 + t10661 / 2.0_f64 - t10663 + t10664 - t10665 + t10666;
    (t10657, t10661, t10667)
}
