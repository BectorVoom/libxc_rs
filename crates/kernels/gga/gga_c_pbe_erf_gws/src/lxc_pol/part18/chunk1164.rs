//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1164/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1164<F: Float>(t54129: F, t54136: F, t54153: F, t54167: F, t56978: F, t56980: F, t56982: F, t56984: F, t56986: F, t56988: F, t56990: F, t56992: F, t56994: F, t1125: F, t54023: F, t3754: F, t51255: F) -> (F, F, F) {
    let t56996 = -t54129 + t56978 / 96.0 + t56980 / 24.0 - 7.0 / 72.0 * t56982 + 5.0 / 384.0 * t56984 - t54136 + t54153 + t56986 / 768.0 - 7.0 / 1152.0 * t56988 + t54167 - 7.0 / 144.0 * t56990 + 7.0 / 72.0 * t56992 + t56994 / 96.0;
    let t56998 = t1125 * t54023;
    let t57000 = t51255 * t3754;
    (t56996, t56998, t57000)
}
