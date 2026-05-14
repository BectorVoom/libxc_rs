//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 316/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk316<F: Float>(t322: F, t1115: F, t499: F, t1048: F, t1072: F) -> (F, F, F) {
    let t324 = 0.0 < t322;
    let t1116 = t499 * t1115;
    let t1118 = t1048 * t1116 / 4.0;
    let t1119 = t1072 / 4.0;
    let t1120 = piecewise3(t324, 0.0, t1119);
    (t1118, t1119, t1120)
}
