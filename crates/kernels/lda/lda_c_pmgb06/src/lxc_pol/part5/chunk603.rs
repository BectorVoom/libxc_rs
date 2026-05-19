//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 603/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk603<F: Float>(t1152: F, t123: F, t566: F, t290: F, t642: F, t247: F, t701: F, t2789: F, t301: F, t83: F, t297: F, t4001: F) -> (F, F, F, F, F, F) {
    let t4257 = t123 * t1152 * t566;
    let t4283 = F::cast_from(1.279801625812305_f64) * t642 * t290;
    let t4284 = t247 * t701;
    let t4294 = t83 * t2789 * t301;
    let t4296 = F::cast_from(0.01197423401025461_f64) * t297 * t4294;
    let t4297 = t4001 * t83;
    (t4257, t4283, t4284, t4294, t4296, t4297)
}
