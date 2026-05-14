//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1200/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1200<F: Float>(t14615: F, t25188: F, t25462: F, t29026: F, t55797: F, t6374: F, t15128: F, t24944: F, t25377: F, t55768: F, t2843: F, t28924: F, t875: F, t1466: F, t193: F, t24964: F, t25397: F, t25402: F, t25480: F, t28863: F, t29033: F, t29040: F, t6210: F, t6963: F, t7024: F, t880: F, t98322: F) -> (F, F, F, F, F, F) {
    let t112399 = t25188 * t14615;
    let t112402 = t25462 * t29026 / 27.0;
    let t112403 = t55797 * t6374;
    let t112405 = t15128 * t24944;
    let t112407 = t55768 * t25377;
    let t112410 = t2843 * t28924 * t875;
    let t112425 = -t6963 * t25402 / 3.0 - 2.0 / 3.0 * t6963 * t25397 + 8.0 * t112399 + t112402 + 8.0 * t112403 + 8.0 * t112405 - 12.0 * t112407 + 8.0 * t112410 + t25480 * t7024 / 6.0 + t6210 * t28863 / 3.0 + t1466 * t193 * t29040 * t880 / 3.0 - 2.0 / 3.0 * t1466 * t193 * t24964 * t29033 + t98322 / 27.0;
    (t112399, t112403, t112405, t112407, t112410, t112425)
}
