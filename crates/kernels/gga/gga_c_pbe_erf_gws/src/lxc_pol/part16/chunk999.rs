//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 999/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk999<F: Float>(t6333: F, t3128: F, t6258: F, t8923: F, t8925: F, t8927: F, t8930: F, t8932: F, t8936: F, t8938: F, t8943: F, t8948: F, t8951: F) -> (F, F, F) {
    let t8952 = F::new(7.0) / F::new(72.0) * t6333;
    let t8954 = t3128 * t6258 / F::new(48.0);
    let t8955 = t8923 - t8925 - t8927 - t8930 + t8932 + t8936 - t8938 - t8943 + t8948 - t8951 + t8952 - t8954;
    (t8952, t8954, t8955)
}
