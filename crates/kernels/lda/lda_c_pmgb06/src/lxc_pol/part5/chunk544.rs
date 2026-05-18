//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 544/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk544<F: Float>(t1193: F, t1354: F, t2822: F, t1186: F, t1343: F, t421: F, t398: F, t740: F, t1183: F, t27: F) -> (F, F, F, F, F) {
    let t2825 = F::new(0.0034679929861433484) * t2822 * t1193 * t1354;
    let t2831 = t1343 * t1186 * t421;
    let t2833 = t740 * t398;
    let t2835 = t2833 * t1193 * t1354;
    let t2837 = t1183 * t27;
    (t2825, t2831, t2833, t2835, t2837)
}
