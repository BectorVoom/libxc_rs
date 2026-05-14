//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 521/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk521<F: Float>(t1421: F, t1459: F, t1511: F, t1513: F, t1526: F, t2810: F, t2813: F, t2816: F, t2866: F, t2869: F, t2872: F, t2879: F, t881: F, t2854: F, t910: F, t2266: F) -> (F, F) {
    let t2881 = t2866 + t1421 - t1511 + t1459 + t2869 - t1526 - 0.2363e1 * t881 * t2816 - 0.2363e1 * t2872 - 0.2363e1 * t881 * t2810 - 0.2363e1 * t881 * t2813 - t1513 + t2879;
    let t2889 = t2854 * t910;
    let t2890 = t2266 * t2889;
    (t2881, t2890)
}
