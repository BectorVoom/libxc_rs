//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 729/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk729<F: Float>(t12705: F, t12707: F, t12713: F, t12715: F, t12719: F, t12721: F, t12725: F, t12726: F, t12728: F, t12733: F, t12735: F, t12737: F, t12739: F, t12741: F, t5933: F, t5944: F, t8440: F) -> (F,) {
    let t12742 = t5933 - t5944 + 8.0 * t8440 - t12705 + t12707 + t12713 + t12715 - t12719 + t12721 + t12725 - t12726 - t12728 + t12733 - t12735 + t12737 + t12739 - t12741;
    (t12742,)
}
