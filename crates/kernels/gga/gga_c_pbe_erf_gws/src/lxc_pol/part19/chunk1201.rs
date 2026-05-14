//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1201/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1201<F: Float>(t14881: F, t353: F, t3721: F, t859: F, t1206: F, t14182: F, t14193: F, t3028: F, t3037: F, t34773: F, t34850: F, t35057: F, t35260: F, t4083: F, t54984: F, t55695: F, t55702: F, t55717: F, t55726: F, t57358: F, t57361: F, t57371: F, t57375: F, t57379: F, t6793: F, t8629: F, t8793: F) -> (F,) {
    let t58540 = t859 * t353 * t14881 * t3721;
    let t58547 = -7.0 / 72.0 * t57358 + t57361 / 384.0 - t55695 - t55702 - t34773 * t859 * t353 * t1206 * t3037 / 48.0 - 7.0 / 1152.0 * t57371 + t57375 / 24.0 + t55726 + t57379 / 8.0 + t34850 * t14193 / 96.0 + t8793 * t55717 / 24.0 + t8793 * t54984 / 24.0 + t8629 * t859 * t353 * t1206 * t3028 / 96.0 - t6793 * t58540 / 16.0 + t35057 * t14182 / 48.0 - t35260 * t4083 / 96.0;
    (t58547,)
}
