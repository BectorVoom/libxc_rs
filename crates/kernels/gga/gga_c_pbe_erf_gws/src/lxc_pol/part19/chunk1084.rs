//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1084/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1084<F: Float>(t11876: F, t11880: F, t11885: F, t11888: F, t11893: F, t11907: F, t11911: F, t11913: F, t11923: F, t11927: F, t9041: F, t9086: F, t9096: F) -> F {
    let t12157 = t11876 - t9041 + t11880 + t11885 - t11888 + t11893 + t9086 - t9096 - t11907 + t11911 + t11913 - t11923 + t11927;
    t12157
}
