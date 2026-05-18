//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 956/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk956<F: Float>(t5948: F, t5949: F, t5952: F, t5954: F, t7697: F, t7702: F, t7708: F, t7710: F, t7712: F, t7715: F, t7719: F, t7724: F, t7740: F, t7742: F, t7744: F, t7749: F, t7750: F) -> F {
    let t8449 = t5948 + F::new(4.0) / F::new(3.0) * t5949 + t5952 - t7697 + t7702 + t7708 + t7710 - t7712 - t7715 + t7719 - t7724 + F::new(0.22363485482220676312e-1) * t5954 - t7740 + t7742 - t7744 + t7749 + t7750;
    t8449
}
