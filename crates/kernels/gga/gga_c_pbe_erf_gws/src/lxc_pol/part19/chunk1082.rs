//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1082/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1082<F: Float>(t11762: F, t11766: F, t11768: F, t11770: F, t11772: F, t11775: F, t11780: F, t11784: F, t11789: F, t11796: F, t8969: F, t8971: F, t8973: F) -> F {
    let t12153 = -t8969 + t8971 + t8973 - t11762 + t11766 - t11768 + t11770 + t11772 - t11775 - t11780 - t11784 + t11789 - t11796;
    t12153
}
