//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 791/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk791<F: Float>(t3764: F, t3766: F, t3867: F, t3871: F, t3874: F, t3877: F, t3892: F, t3911: F, t4550: F, t4552: F, t4554: F, t4559: F) -> F {
    let t7422 = -t3764 - t3766 + F::cast_from(0.03253074390090522_f64) * t4550 + F::cast_from(3.5089341735807875_f64) * t4552 - F::cast_from(51.94757731704439_f64) * t4554 - F::cast_from(1.7544670867903938_f64) * t4559 + t3892 - t3867 + t3871 + t3874 + t3911 + t3877;
    t7422
}
