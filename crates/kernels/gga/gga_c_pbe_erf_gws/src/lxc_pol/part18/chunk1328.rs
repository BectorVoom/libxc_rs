//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1328/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1328<F: Float>(t3816: F, t51371: F, t1125: F, t54101: F, t11991: F, t14011: F, t54129: F, t54136: F, t54153: F, t54167: F, t56978: F, t56980: F, t56982: F, t56984: F, t56986: F, t56988: F) -> F {
    let t56990 = t51371 * t3816;
    let t56992 = t1125 * t54101;
    let t56994 = t14011 * t11991;
    let t56996 = -t54129 + t56978 / F::cast_from(96.0_f64) + t56980 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t56982 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t56984 - t54136 + t54153 + t56986 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t56988 + t54167 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t56990 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t56992 + t56994 / F::cast_from(96.0_f64);
    t56996
}
