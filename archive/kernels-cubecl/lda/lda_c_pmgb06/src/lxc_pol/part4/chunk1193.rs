//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1193/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1193<F: Float>(t11864: F, t11866: F, t13788: F, t2064: F, t439: F, t477: F, t822: F, t11868: F, t1385: F, t1868: F, t2010: F, t5168: F, t6372: F) -> (F, F, F, F, F, F) {
    let t15747 = F::cast_from(128.0_f64) / F::cast_from(405.0_f64) * t11864;
    let t15748 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t11866;
    let t15753 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t439 * t13788 * t822 * t477 * t2064;
    let t15754 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t11868;
    let t15758 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2010 * t1385 * t1868 * t2064;
    let t15760 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5168 * t6372;
    (t15747, t15748, t15753, t15754, t15758, t15760)
}
