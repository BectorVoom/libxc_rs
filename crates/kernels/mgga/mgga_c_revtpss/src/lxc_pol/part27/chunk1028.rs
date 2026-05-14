//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1028/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1028<F: Float>(t26124: F, t572: F, t2371: F, t7330: F, t117: F, t25832: F, t10301: F, t7565: F, t38: F, t7574: F, t2247: F, t2282: F, t55: F, t2251: F, t2258: F, t25137: F, t7571: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26126 = 12.0 * t572 * t26124;
    let t26127 = t7330 * t2371;
    let t26129 = 6.0 * t572 * t26127;
    let t26130 = t117 * t25832;
    let t26132 = 3.0 * t572 * t26130;
    let t26749 = t10301 * t7565;
    let t26754 = t38 * t7574;
    let t26755 = t2247 * t26754;
    let t26776 = t55 * t2282;
    let t26781 = 5.0 / 18.0 * t26776 * t2251 - 5.0 / 6.0 * t7571 * t2258 - t25137;
    (t26126, t26127, t26129, t26130, t26132, t26749, t26754, t26755, t26776, t26781)
}
