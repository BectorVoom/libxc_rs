//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 977/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk977<F: Float>(t1227: F, t315: F, t934: F, t3566: F, t3576: F, t28: F, t3: F, t37: F, t27: F, t4238: F, t55: F, t3502: F, param_hyb_omega_0: F) -> (F, F, F, F, F) {
    let t8323 = t934 * t315 * t1227;
    let t8324 = t3566 * t8323;
    let t8328 = t3576 * t8323;
    let t8333 = F::new(1.0) / t37 / t28 / t3 / F::new(48.0);
    let t8337 = t4238 * t27 * t55;
    let t8339 = F::cast_from(1.6239027777777777_f64) * param_hyb_omega_0 * t8333 * t3502 * t8337;
    (t8324, t8328, t8333, t8337, t8339)
}
