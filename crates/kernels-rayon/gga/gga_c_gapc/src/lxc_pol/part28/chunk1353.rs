//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1353/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1353(t10529: f64, t8616: f64, t12285: f64, t7056: f64, t35379: f64, t35384: f64, t35386: f64, t35388: f64, t35390: f64, t35393: f64, t35395: f64, t35400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36326 = 2.0_f64 * t10529 * t8616;
    let t36331 = 4.0_f64 * t7056 * t12285;
    let t36332 = 0.6951859425083008306e-3_f64 * t35379;
    let t36333 = 0.24882710529037792555e-6_f64 * t35384;
    let t36334 = 0.86898242813537603825e-4_f64 * t35386;
    let t36335 = 0.17379648562707520765e-3_f64 * t35388;
    let t36336 = 0.17379648562707520765e-2_f64 * t35390;
    let t36337 = 0.45552534985326748556e-4_f64 * t35393;
    let t36338 = 0.6951859425083008306e-3_f64 * t35395;
    let t36340 = 0.6951859425083008306e-3_f64 * t35400;
    (t36326, t36331, t36332, t36333, t36334, t36335, t36336, t36337, t36338, t36340)
}
