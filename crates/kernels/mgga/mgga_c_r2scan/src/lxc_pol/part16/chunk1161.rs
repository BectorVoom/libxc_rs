//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1161/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1161<F: Float>(t1065: F, t39197: F, t42878: F, t42877: F, t792: F, t39190: F, t795: F, t37327: F, t4176: F, t14656: F, t986: F, t3270: F) -> (F, F, F, F) {
    let t42881 = F::new(15.0) / F::new(4.0) * t39197 * t1065 * t42878;
    let t42882 = t42877 * t792;
    let t42885 = F::new(135.0) / F::new(32.0) * t39190 * t1065 * t42882;
    let t42886 = t42877 * t795;
    let t42889 = F::new(15.0) / F::new(8.0) * t37327 * t4176 * t42886;
    let t42890 = t14656 * t986;
    let t42891 = t3270 * t42890;
    (t42881, t42885, t42889, t42891)
}
