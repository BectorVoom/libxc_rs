//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 474/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk474<F: Float>(t317: F, t6261: F, t193: F, t1477: F, t880: F, t1483: F, t1882: F, t1476: F, t312: F) -> (F, F, F, F, F, F) {
    let t6262 = t6261 * t317;
    let t6263 = t193 * t6262;
    let t6266 = t1477 * t880;
    let t6267 = t193 * t6266;
    let t6272 = t1882 * t1483 / 9.0;
    let t6273 = t312 * t1476;
    (t6262, t6263, t6266, t6267, t6272, t6273)
}
