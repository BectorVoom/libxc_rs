//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 731/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk731<F: Float>(t3764: F, t3766: F, t3867: F, t3871: F, t3874: F, t3877: F, t3892: F, t3911: F, t4550: F, t4552: F, t4554: F, t4559: F, t3881: F, t3884: F, t3888: F, t3942: F, t3944: F, t3946: F, t3949: F, t3954: F, t3956: F, t3959: F, t3962: F, t3965: F, t3968: F) -> (F, F) {
    let t7422 = -t3764 - t3766 + 0.03253074390090522 * t4550 + 3.5089341735807875 * t4552 - 51.94757731704439 * t4554 - 1.7544670867903938 * t4559 + t3892 - t3867 + t3871 + t3874 + t3911 + t3877;
    let t7423 = t3881 - t3884 - t3888 + t3942 - t3944 - t3946 - t3949 - t3954 + t3956 - t3959 - t3962 + t3965 + t3968;
    (t7422, t7423)
}
