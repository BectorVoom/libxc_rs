//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1064/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1064<F: Float>(t2323: F, t3875: F, t3128: F, t8833: F, t2255: F, t3757: F, t9364: F, t3257: F, t3803: F, t6345: F, t11576: F, t3131: F, t3139: F) -> (F, F, F, F, F) {
    let t11944 = t2323 * t3875;
    let t11946 = t3128 * t8833;
    let t11947 = F::new(7.0) / F::new(72.0) * t11946;
    let t11949 = t2255 * t9364 * t3757;
    let t11953 = t3257 * t3803 * t6345;
    let t11957 = t3139 * t3131 * t11576;
    (t11944, t11947, t11949, t11953, t11957)
}
