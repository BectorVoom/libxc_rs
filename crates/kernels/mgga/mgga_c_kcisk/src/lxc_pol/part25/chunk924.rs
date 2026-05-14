//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 924/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk924<F: Float>(t1644: F, t6799: F, t1665: F, t4737: F, t6802: F, t2368: F, t4741: F, t4745: F, t16309: F, t16313: F, t16316: F, t16321: F, t16323: F, t16325: F, t16328: F, t16331: F, t16334: F, t16338: F, t16341: F, t16345: F, t16346: F, t1674: F, t4791: F, t6851: F) -> (F, F, F, F) {
    let t16351 = t6799 * t1644;
    let t16353 = 2.0 * t16351 * t1665;
    let t16355 = 1.0 * t6802 * t4737;
    let t16356 = t2368 * t4741;
    let t16358 = 0.16081824322151104822e2 * t16356 * t4745;
    let t16359 = -0.1025389702100779493e4 * t1674 * t16309 + 0.1038945353962551798e3 * t1674 * t16313 + 0.11696446794910408142e1 * t1674 * t16316 + t16321 - t16323 + t16325 - t16328 - t16331 - t16334 + t16338 + t16341 + t16345 - 0.35089340384731224426e1 * t1674 * t16346 - 0.17315755899375863299e2 * t6851 * t4791 + t16353 + t16355 + t16358;
    (t16353, t16355, t16358, t16359)
}
