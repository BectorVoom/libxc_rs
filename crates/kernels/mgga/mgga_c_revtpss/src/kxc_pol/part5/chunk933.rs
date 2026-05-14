//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 933/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk933<F: Float>(t2236: F, t3: F, t25: F, t2246: F, t599: F, t88: F, t89: F, t90: F, t29: F, t46: F, t47: F, t58: F, t59: F, t10199: F, t2851: F, t78: F) -> (F, F, F, F, F, F, F) {
    let t10292 = t2236 * t3;
    let t10293 = 1.0 / t10292;
    let t10295 = 336.0 * t25 * t10293;
    let t10301 = t599 * t2246;
    let t10308 = 1.0 / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    let t10355 = 1.0 / t47 / t46;
    let t10368 = 1.0 / t59 / t58;
    let t10379 = 1232.0 / 27.0 * t10199;
    let t10389 = 1.0 / t78 / t2851;
    (t10295, t10301, t10309, t10355, t10368, t10379, t10389)
}
