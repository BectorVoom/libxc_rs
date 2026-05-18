//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1351/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1351<F: Float>(t10529: F, t8616: F, t12285: F, t7056: F, t35379: F, t35384: F, t35386: F, t35388: F, t35390: F, t35393: F, t35395: F, t35400: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36326 = F::new(2.0) * t10529 * t8616;
    let t36331 = F::new(4.0) * t7056 * t12285;
    let t36332 = F::new(0.6951859425083008306e-3) * t35379;
    let t36333 = F::new(0.24882710529037792555e-6) * t35384;
    let t36334 = F::new(0.86898242813537603825e-4) * t35386;
    let t36335 = F::new(0.17379648562707520765e-3) * t35388;
    let t36336 = F::new(0.17379648562707520765e-2) * t35390;
    let t36337 = F::new(0.45552534985326748556e-4) * t35393;
    let t36338 = F::new(0.6951859425083008306e-3) * t35395;
    let t36340 = F::new(0.6951859425083008306e-3) * t35400;
    (t36326, t36331, t36332, t36333, t36334, t36335, t36336, t36337, t36338, t36340)
}
