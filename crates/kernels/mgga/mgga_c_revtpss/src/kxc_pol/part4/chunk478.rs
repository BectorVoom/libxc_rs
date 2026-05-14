//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 478/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk478<F: Float>(t1131: F, t1733: F, t1154: F, t1717: F, t1163: F, t1166: F, t1724: F, t1727: F, t1730: F, t1169: F) -> (F, F, F, F) {
    let t1735 = 1.0 * t1131 * t1733;
    let t1737 = -t1154 + 0.17123333333333333333e-1 * t1717;
    let t1744 = 0.3529725e1 * t1724 - t1163 + 0.516475e0 * t1717 + 0.6311625e0 * t1727 - t1166 + 0.104195e0 * t1730;
    let t1745 = t1744 * t1169;
    (t1735, t1737, t1744, t1745)
}
