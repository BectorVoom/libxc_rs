//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1222/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1222<F: Float>(t37769: F, t9373: F, t3309: F, t9327: F, t2147: F, t29936: F, t3332: F, t11683: F, t26088: F, t10760: F, t29946: F, t6535: F) -> (F, F, F, F, F) {
    let t43586 = t37769 * t9373;
    let t43588 = t9327 * t3309;
    let t43592 = t2147 * t3332 * t29936;
    let t43594 = t26088 * t11683;
    let t43597 = t6535 * t10760 * t29946;
    (t43586, t43588, t43592, t43594, t43597)
}
