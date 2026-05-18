//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1084/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1084<F: Float>(t11810: F, t11812: F, t11816: F, t11818: F, t11833: F, t11838: F, t11844: F, t11862: F, t11863: F, t11867: F, t11870: F, t11874: F) -> F {
    let t12156 = -t11810 + t11812 - t11816 - t11818 + t11833 + t11838 + t11844 + t11862 - t11863 - t11867 + t11870 - t11874;
    t12156
}
