//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1023/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1023<F: Float>(t2362: F, t2379: F, t2408: F, t3079: F, t3207: F, t335: F, t6156: F, t6173: F, t6793: F, t6797: F, t8654: F, t8776: F, t8780: F, t8784: F, t8790: F, t8793: F, t8797: F, t8803: F, t8806: F, t8810: F, t8812: F, t8818: F, t9203: F, t9208: F) -> F {
    let t9211 = -t8776 * t2362 / F::new(32.0) + t8780 + t8784 * t3079 / F::new(96.0) + t6793 * t8790 / F::new(24.0) + t8793 * t6797 / F::new(24.0) + t2408 * t8797 / F::new(24.0) - t8803 + t3207 * t8806 / F::new(8.0) - t8810 + t2408 * t8812 / F::new(24.0) + F::new(7.0) / F::new(288.0) * t6156 - t8654 * t2379 / F::new(48.0) - F::new(35.0) / F::new(432.0) * t8818 + t335 * t9203 / F::new(96.0) - F::new(7.0) / F::new(144.0) * t6173 - t335 * t9208 / F::new(96.0);
    t9211
}
