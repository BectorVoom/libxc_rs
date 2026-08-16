//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1090/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1090<F: Float>(t34132: F, t34166: F, t118: F, t13272: F, t8619: F, t1497: F, t8621: F, t8622: F, t1469: F, t32591: F, t8442: F, t1493: F, t32600: F) -> (F, F, F, F, F, F) {
    let t34167 = t34132 + t34166;
    let t34168 = t118 * t34167;
    let t34169 = t13272 * t8619;
    let t34173 = t8621 * t8622 * t1497;
    let t34176 = t32591 * t1469;
    let t34177 = t8442 * t34176;
    let t34181 = t8621 * t32600 * t1493;
    (t34167, t34168, t34169, t34173, t34177, t34181)
}
