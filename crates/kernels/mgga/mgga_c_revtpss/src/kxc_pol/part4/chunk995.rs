//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 995/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk995<F: Float>(t88: F, t89: F, t90: F, t29: F, t46: F, t47: F, t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F) -> (F, F, F, F, F, F) {
    let t10308 = F::cast_from(1.0_f64) / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    let t10355 = F::cast_from(1.0_f64) / t47 / t46;
    let t10368 = F::cast_from(1.0_f64) / t59 / t58;
    let t10379 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t10199;
    let t10389 = F::cast_from(1.0_f64) / t78 / t2851;
    let t10398 = F::cast_from(1.0_f64) / t81 / t3361;
    (t10309, t10355, t10368, t10379, t10389, t10398)
}
