//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1059/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1059<F: Float>(t1218: F, t1466: F, t1479: F, t1506: F, t25487: F, t29419: F, t301: F, t31665: F, t31669: F, t31674: F, t31679: F, t31683: F, t31688: F, t31761: F, t31824: F, t31836: F, t31842: F, t31937: F, t31945: F, t31952: F, t31956: F, t31963: F, t5207: F, t6963: F, t7024: F, t7129: F) -> (F,) {
    let t31971 = -2.0 * t1218 * t7129 + t1466 * t31665 / 6.0 + t1466 * t31669 / 3.0 + t1466 * t31674 / 6.0 - t1466 * t31679 / 3.0 - 2.0 / 3.0 * t1466 * t31683 - 2.0 / 3.0 * t1466 * t31688 - t301 * t31952 - 2.0 * t31761 - t5207 * t1506 + 2.0 * t31956 + t25487 - 12.0 * t31937 + t6963 * t7024 / 3.0 + t31963 * t1479 / 6.0 - 2.0 * t31824 - t29419 / 9.0 + 8.0 * t31945 + 4.0 * t31842 + 8.0 * t31836;
    (t31971,)
}
