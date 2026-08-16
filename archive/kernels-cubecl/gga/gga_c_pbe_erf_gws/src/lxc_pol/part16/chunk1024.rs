//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1024/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1024<F: Float>(t2362: F, t2379: F, t2408: F, t3079: F, t3207: F, t335: F, t6156: F, t6173: F, t6793: F, t6797: F, t8654: F, t8776: F, t8780: F, t8784: F, t8790: F, t8793: F, t8797: F, t8803: F, t8806: F, t8810: F, t8812: F, t8818: F, t9203: F, t9208: F) -> F {
    let t9211 = -t8776 * t2362 / F::cast_from(32.0_f64) + t8780 + t8784 * t3079 / F::cast_from(96.0_f64) + t6793 * t8790 / F::cast_from(24.0_f64) + t8793 * t6797 / F::cast_from(24.0_f64) + t2408 * t8797 / F::cast_from(24.0_f64) - t8803 + t3207 * t8806 / F::cast_from(8.0_f64) - t8810 + t2408 * t8812 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6156 - t8654 * t2379 / F::cast_from(48.0_f64) - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t8818 + t335 * t9203 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6173 - t335 * t9208 / F::cast_from(96.0_f64);
    t9211
}
