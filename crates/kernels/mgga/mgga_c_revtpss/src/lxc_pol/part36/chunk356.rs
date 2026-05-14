//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 356/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk356<F: Float>(t1132: F, t1723: F, t1139: F, t1145: F, t1715: F, t141: F, t1137: F, t1144: F, t1717: F, t1150: F, t1131: F, t1154: F, t1163: F, t1166: F, t1169: F, t1173: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1724 = t1132 * t1723;
    let t1727 = t1139 * t1723;
    let t1729 = t1145 * t1715;
    let t1730 = t141 * t1729;
    let t1732 = 0.1898925e1 * t1724 - t1137 + 0.29896666666666666667e0 * t1717 + 0.3071625e0 * t1727 - t1144 + 0.82156666666666666667e-1 * t1730;
    let t1733 = t1732 * t1150;
    let t1735 = 1.0 * t1131 * t1733;
    let t1737 = -t1154 + 0.17123333333333333333e-1 * t1717;
    let t1744 = 0.3529725e1 * t1724 - t1163 + 0.516475e0 * t1717 + 0.6311625e0 * t1727 - t1166 + 0.104195e0 * t1730;
    let t1745 = t1744 * t1169;
    let t1749 = -t1173 + 0.92708333333333333333e-2 * t1717;
    (t1724, t1727, t1729, t1730, t1732, t1733, t1735, t1737, t1744, t1745, t1749)
}
