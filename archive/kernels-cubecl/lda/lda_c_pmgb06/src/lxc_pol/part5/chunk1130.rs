//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1130/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1130<F: Float>(t17617: F, t1893: F, t5077: F, t1864: F, t2630: F, t1859: F, t5083: F, t15862: F, t6562: F, t6630: F, t15865: F, t6633: F) -> (F, F, F, F, F, F) {
    let t20569 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t17617 * t1893;
    let t20572 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t2630 * t1864;
    let t20575 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5083 * t2630 * t1859;
    let t20577 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t15862 * t6562;
    let t20579 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t15862 * t6630;
    let t20581 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15865 * t6633;
    (t20569, t20572, t20575, t20577, t20579, t20581)
}
