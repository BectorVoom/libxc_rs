//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 588/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk588<F: Float>(t286: F, t3952: F, t1108: F, t687: F, t110: F, t980: F, t1121: F, t410: F, t698: F, t959: F, t968: F, t30: F, t653: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3954 = F::new(120.0) * t3952 * t286;
    let t3955 = t1108 * t687;
    let t3956 = F::new(96.0) * t3955;
    let t3957 = t110 * t980;
    let t3959 = F::new(0.03253074390090522) * t1121 * t3957;
    let t3960 = t410 * t698;
    let t3962 = F::new(0.02168716260060348) * t1121 * t3960;
    let t3963 = t110 * t959;
    let t3965 = F::new(0.01626537195045261) * t1121 * t3963;
    let t3966 = t110 * t968;
    let t3968 = F::new(0.4815973313767657) * t1121 * t3966;
    let t3969 = t653 * t30;
    (t3954, t3955, t3956, t3957, t3959, t3960, t3962, t3963, t3965, t3966, t3968, t3969)
}
