//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1143/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1143<F: Float>(t108: F, t1286: F, t1564: F, t15772: F, t15885: F, t22873: F, t22907: F, t22917: F, t22922: F, t22935: F, t25856: F, t28: F, t29586: F, t29590: F, t29605: F, t29729: F, t29731: F, t29748: F, t29750: F, t379: F, t4462: F, t46565: F, t492: F, t5495: F, t5501: F, t5502: F, t5507: F, t8418: F, t93392: F) -> (F,) {
    let t116217 = -2.0 / 3.0 * t5495 * t29731 - 2.0 / 3.0 * t1286 * t28 * t22873 * t29729 - t5501 * t1564 * t22922 * t4462 / 18.0 - t5501 * t1564 * t5502 * t15772 / 18.0 - t22935 * t29590 / 27.0 - t1286 * t28 * t5507 * t108 * t15885 / 3.0 + 2.0 / 9.0 * t5501 * t22907 * t29729 * t379 + t5495 * t29750 + t1286 * t28 * t93392 * t29748 - 24.0 * t8418 * t29605 * t492 - 24.0 * t46565 * t25856 - t22935 * t29586 / 18.0 - t5501 * t1564 * t22917 * t4462 / 18.0;
    (t116217,)
}
