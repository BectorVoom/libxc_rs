//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 988/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk988<F: Float>(t5: F, t4147: F, t8107: F, t1497: F, t8621: F, t8881: F, t1469: F, t33268: F, t8442: F, t1493: F, t33275: F, t32798: F, t32802: F, t33283: F, t34402: F, t34410: F, t8737: F, t8882: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t34495 = t4147 * t8107;
    let t34761 = t8621 * t8881 * t1497;
    let t34764 = t33268 * t1469;
    let t34765 = t8442 * t34764;
    let t34771 = t8621 * t33275 * t1493;
    let t34775 = piecewise3(t8, 0.0, -5.0 / 72.0 * t34402 * t8882 + 5.0 / 12.0 * t32798 * t34761 + 5.0 / 18.0 * t32802 * t34765 - 5.0 / 72.0 * t34410 * t8882 - 5.0 / 36.0 * t8737 * t34771 + t33283);
    (t34495, t34761, t34765, t34771, t34775)
}
