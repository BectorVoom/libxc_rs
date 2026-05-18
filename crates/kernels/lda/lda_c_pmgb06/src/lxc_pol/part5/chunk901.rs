//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 901/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk901<F: Float>(t421: F, t4244: F, t4247: F, t1186: F, t2847: F, t1354: F, t2822: F, t2841: F, t4240: F, t4298: F, t10644: F, t118: F) -> (F, F, F, F, F, F, F) {
    let t10825 = t4244 * t421;
    let t10828 = F::new(0.10359818039161417) * t4247 * t421;
    let t10834 = t2847 * t1186 * t421;
    let t10838 = F::new(0.013871971944573394) * t2822 * t2841 * t1354;
    let t10840 = F::new(0.12408369628826103) * t4240 * t421;
    let t10843 = F::new(0.02267957317922317) * t4298 * t1354;
    let t10844 = t10644 * t118;
    (t10825, t10828, t10834, t10838, t10840, t10843, t10844)
}
