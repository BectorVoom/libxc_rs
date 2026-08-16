//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1213/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1213(t10326: f64, t10344: f64, t10356: f64, t11231: f64, t1923: f64, t1927: f64, t2251: f64, t2258: f64, t25117: f64, t25129: f64, t25132: f64, t25140: f64, t25157: f64, t25162: f64, t25164: f64, t49: f64, t606: f64, t6954: f64, t6968: f64, t6974: f64, t6978: f64, t72: f64, t92565: f64, t92568: f64, t92570: f64, t92573: f64, t92577: f64, t92581: f64, t92585: f64, t92588: f64, t92597: f64, t92600: f64, t92605: f64, t92612: f64) -> f64 {
    let t92618 = -10.0_f64 * t92565 * t25164 + 30.0_f64 * t92568 * t92570 - 10.0_f64 * t25162 * t92573 - 10.0_f64 * t25162 * t92577 - 15.0_f64 * t25157 * t92581 - 5.0_f64 * t25162 * t92585 - 5.0_f64 * t92588 * t25164 + t25117 * t6974 + t25117 * t6978 - t6954 * t25140 / 2.0_f64 - t1923 * (-1232.0_f64 / 27.0_f64 * t10344 * t49 + 220.0_f64 / 9.0_f64 * t92597 * t606 - 20.0_f64 / 9.0_f64 * t92600 * t2251 - 20.0_f64 / 3.0_f64 * t25129 * t2258 - 5.0_f64 / 108.0_f64 * t92605 * t10356 + 5.0_f64 / 6.0_f64 * t25132 * t11231 + 5.0_f64 / 6.0_f64 * t6968 * t10326 + t92612) * t72 * t1927 / 6.0_f64;
    t92618
}
