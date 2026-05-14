//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1093/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1093<F: Float>(t2275: F, t613: F, t10355: F, t43: F, t843: F, t10326: F, t10344: F, t10356: F, t11231: F, t1923: F, t1927: F, t2251: F, t2258: F, t25117: F, t25129: F, t25132: F, t25140: F, t25157: F, t25162: F, t25164: F, t49: F, t606: F, t6954: F, t6968: F, t6974: F, t6978: F, t72: F, t92565: F, t92568: F, t92570: F, t92573: F, t92577: F, t92581: F, t92585: F, t92588: F, t92597: F) -> (F,) {
    let t92600 = t613 * t2275;
    let t92605 = t43 * t10355;
    let t92612 = 1232.0 / 27.0 * t843;
    let t92618 = -10.0 * t92565 * t25164 + 30.0 * t92568 * t92570 - 10.0 * t25162 * t92573 - 10.0 * t25162 * t92577 - 15.0 * t25157 * t92581 - 5.0 * t25162 * t92585 - 5.0 * t92588 * t25164 + t25117 * t6974 + t25117 * t6978 - t6954 * t25140 / 2.0 - t1923 * (-1232.0 / 27.0 * t10344 * t49 + 220.0 / 9.0 * t92597 * t606 - 20.0 / 9.0 * t92600 * t2251 - 20.0 / 3.0 * t25129 * t2258 - 5.0 / 108.0 * t92605 * t10356 + 5.0 / 6.0 * t25132 * t11231 + 5.0 / 6.0 * t6968 * t10326 + t92612) * t72 * t1927 / 6.0;
    (t92618,)
}
