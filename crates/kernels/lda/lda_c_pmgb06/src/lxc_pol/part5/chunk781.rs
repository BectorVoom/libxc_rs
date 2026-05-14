//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 781/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk781<F: Float>(t342: F, t740: F, t934: F, t3576: F, t28: F, t3: F, t37: F, t27: F, t4238: F, t55: F, t3502: F, t3500: F, t3510: F, t61: F, t1179: F, t1276: F) -> (F, F, F, F, F, F, F, F) {
    let t8305 = t934 * t740 * t342;
    let t8306 = t3576 * t8305;
    let t8333 = 1.0 / t37 / t28 / t3 / 48.0;
    let t8337 = t4238 * t27 * t55;
    let t8339 = 1.6239027777777777 * param_hyb_omega_0 * t8333 * t3502 * t8337;
    let t8346 = 0.16322666666666666 * t61 * t3500 * t3510 * t8337;
    let t8352 = t55 * t1179 * t342;
    let t8353 = t1276 * t8352;
    (t8305, t8306, t8333, t8337, t8339, t8346, t8352, t8353)
}
