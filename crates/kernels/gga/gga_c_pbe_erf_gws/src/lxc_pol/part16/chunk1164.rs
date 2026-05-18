//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1164/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1164<F: Float>(t14043: F, t14048: F, t14529: F, t14531: F, t14533: F, t14536: F, t14539: F, t14542: F, t14544: F, t14549: F, t15070: F, t14554: F) -> (F, F) {
    let t15071 = -t14529 / F::new(384.0) - t14531 / F::new(96.0) - t14533 / F::new(24.0) - t14536 / F::new(24.0) - t14539 / F::new(48.0) + t14043 - t14542 / F::new(24.0) + t14544 / F::new(384.0) + t14048 + t14549 / F::new(8.0) - t15070;
    let t15072 = F::new(7.0) / F::new(144.0) * t14554;
    (t15071, t15072)
}
