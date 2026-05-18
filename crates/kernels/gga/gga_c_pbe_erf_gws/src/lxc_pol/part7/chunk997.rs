//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 997/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk997<F: Float>(t5385: F, t720: F, t1365: F, t252: F, t254: F, t16569: F, t5560: F, t1906: F, t1923: F, t256: F, t5426: F, t707: F) -> (F, F, F, F, F) {
    let t18240 = F::new(32.0) / F::new(81.0) * t720 * t5385;
    let t18243 = F::new(56.0) / F::new(243.0) * t252 * t254 * t1365;
    let t18245 = F::new(0.80823369382716049381e-2) * t16569 * t5560;
    let t18247 = t1906 * t1923 * t256;
    let t18250 = t707 * t5426 * t256;
    (t18240, t18243, t18245, t18247, t18250)
}
