//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 894/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk894<F: Float>(t5417: F, t5418: F, t5423: F, t5429: F, t5430: F, t5433: F, t5436: F, t7740: F, t7742: F, t7744: F, t7749: F, t7750: F, t7753: F, t7755: F, t7757: F, t7762: F, t7764: F) -> F {
    let t7768 = -t7740 + t7742 - t7744 + t7749 + t7750 - t7753 + t7755 + t7757 - t7762 - t7764 + F::new(2.0) / F::new(3.0) * t5417 + F::new(0.2431111111111111111e0) * t5418 + t5423 + t5429 + F::new(8.0) / F::new(9.0) * t5430 + t5433 + t5436;
    t7768
}
