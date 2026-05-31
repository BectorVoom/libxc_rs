//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1150/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1150<F: Float>(t26120: F, t572: F, t116: F, t7002: F, t670: F, t2371: F, t7330: F, t117: F, t25832: F, t10301: F, t7565: F, t38: F, t7574: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26122 = F::cast_from(6.0_f64) * t572 * t26120;
    let t26123 = t116 * t7002;
    let t26124 = t26123 * t670;
    let t26126 = F::cast_from(12.0_f64) * t572 * t26124;
    let t26127 = t7330 * t2371;
    let t26129 = F::cast_from(6.0_f64) * t572 * t26127;
    let t26130 = t117 * t25832;
    let t26132 = F::cast_from(3.0_f64) * t572 * t26130;
    let t26749 = t10301 * t7565;
    let t26754 = t38 * t7574;
    (t26122, t26123, t26124, t26126, t26127, t26129, t26130, t26132, t26749, t26754)
}
