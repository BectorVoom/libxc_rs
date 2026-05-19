//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 885/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk885<F: Float>(t2060: F, t526: F, t1414: F, t147: F, t163: F, t146: F, t164: F, t9712: F, t9501: F, t1980: F, t604: F, t223: F, t5210: F) -> (F, F, F, F, F, F) {
    let t9938 = t2060 * t526;
    let t9967 = t147 / t163 / t1414;
    let t9981 = F::cast_from(0.10864197530864197_f64) * t146 * t9712 * t164;
    let t9986 = F::cast_from(0.3732469135802469_f64) * t9501;
    let t10079 = t604 * t1980;
    let t10082 = F::new(56.0) / F::new(1215.0) * t223 * t5210;
    (t9938, t9967, t9981, t9986, t10079, t10082)
}
