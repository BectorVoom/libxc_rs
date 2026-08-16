//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 922/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk922<F: Float>(t10662: F, t3270: F, t3269: F, t105: F, t494: F, t97: F) -> (F, F, F, F) {
    let t10663 = t3270 * t10662;
    let t10664 = t3269 * t10663;
    let t10665 = t10664 / F::cast_from(2.0_f64);
    let t10666 = t105 * t494;
    let t10667 = t97 * t10666;
    (t10663, t10665, t10666, t10667)
}
