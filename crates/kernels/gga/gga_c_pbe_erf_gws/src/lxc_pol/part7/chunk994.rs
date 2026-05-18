//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 994/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk994<F: Float>(t17175: F, t17179: F, t17187: F, t17190: F, t17193: F, t17196: F, t17200: F, t17202: F, t17205: F, t17208: F, t17211: F, t17215: F) -> F {
    let t18226 = t17175 + t17179 + t17187 + t17190 + t17193 - t17196 - t17200 + t17202 + t17205 + t17208 + t17211 + t17215;
    t18226
}
