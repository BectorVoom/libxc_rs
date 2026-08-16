//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 413/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk413<F: Float>(t206: F, t686: F, t664: F, t689: F, t1719: F, t226: F, t1835: F, t76: F) -> (F, F, F, F, F) {
    let t1937 = t686 * t206;
    let t1938 = t689 * t664;
    let t1939 = t1937 * t1938;
    let t1942 = t226 * t1719;
    let t1945 = t76 * t1835;
    (t1937, t1938, t1939, t1942, t1945)
}
