//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 745/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk745<F: Float>(t108: F, t489: F, t119: F, t6100: F, t122: F, t507: F, t162: F, t500: F, t277: F, t783: F, t785: F, t1610: F, t1616: F) -> (F, F, F, F, F, F, F) {
    let t6243 = t489 * t108;
    let t6257 = t6100 * t119;
    let t6260 = F::cast_from(0.98171973930797904389e-1_f64) * t6257 * t122 * t507;
    let t6261 = t162 * t500;
    let t6262 = F::cast_from(1.0_f64) / t6261;
    let t6263 = t6262 * t277;
    let t6266 = F::cast_from(0.73613752582167450608e0_f64) * t783 * t785 * t6263;
    let t6268 = t783 * t1610 * t1616;
    (t6243, t6257, t6260, t6261, t6262, t6266, t6268)
}
