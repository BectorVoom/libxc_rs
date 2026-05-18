//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 993/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk993<F: Float>(t2407: F, t8896: F, t6672: F, t2142: F, t3120: F, t332: F, t6238: F, t863: F, t2156: F, t6241: F, t3131: F, t3139: F) -> (F, F, F, F, F, F) {
    let t8897 = t2407 * t8896;
    let t8899 = t6672 * t8897 / F::new(24.0);
    let t8901 = F::new(7.0) / F::new(144.0) * t3120 * t2142;
    let t8903 = t863 * t6238 * t332;
    let t8904 = t6241 * t2156;
    let t8906 = t3139 * t3131 * t8904;
    (t8897, t8899, t8901, t8903, t8904, t8906)
}
