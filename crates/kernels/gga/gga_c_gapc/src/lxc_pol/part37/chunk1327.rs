//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1327/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1327<F: Float>(t10346: F, t134: F, t2207: F, t35834: F, t10301: F, t2580: F, t9497: F, t17874: F, t35382: F, t35766: F, t10237: F, t3729: F) -> (F, F, F, F) {
    let t35875 = t10346 * t2207 * t134 * t35834;
    let t35878 = t10301 * t2580 * t9497;
    let t35881 = t35766 * t35382 * t17874;
    let t35883 = t10237 * t3729;
    (t35875, t35878, t35881, t35883)
}
