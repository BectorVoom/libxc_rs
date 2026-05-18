//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 853/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk853<F: Float>(t2576: F, t4913: F, t2722: F, t626: F, t422: F, t1815: F, t639: F, t5357: F, t561: F, t213: F, t174: F, t838: F) -> (F, F, F, F) {
    let t7223 = F::new(16.0) / F::new(45.0) * t4913 * t2576;
    let t7224 = t2722 * t626;
    let t7225 = t7224 * t422;
    let t7226 = t1815 * t7225;
    let t7228 = F::new(8.0) / F::new(45.0) * t639 * t7226;
    let t7230 = F::new(4.0) / F::new(15.0) * t561 * t5357;
    let t7231 = t213 * t626;
    let t7233 = t174 * t838 * t7231;
    (t7223, t7228, t7230, t7233)
}
