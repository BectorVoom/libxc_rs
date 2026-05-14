//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 790/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk790<F: Float>(t36091: F, t4: F, t26: F, t1477: F, t7129: F, t193: F, t4246: F, t7679: F, t35972: F, t798: F, t317: F, t1091: F, t2665: F, t33996: F, t1476: F, t7124: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36092 = t36091 * t4;
    let t36093 = t36092 * t26;
    let t36096 = t1477 * t7129;
    let t36097 = t193 * t36096;
    let t36101 = t4246 * t7679;
    let t36103 = t798 * t35972;
    let t36104 = t36103 * t317;
    let t36105 = t193 * t36104;
    let t36109 = t2665 * t33996 * t1091;
    let t36112 = t1476 * t7124;
    (t36092, t36093, t36096, t36097, t36101, t36103, t36104, t36105, t36109, t36112)
}
