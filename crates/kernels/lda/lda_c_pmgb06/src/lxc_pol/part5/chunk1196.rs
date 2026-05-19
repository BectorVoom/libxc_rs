//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1196/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1196<F: Float>(t4359: F, t7344: F, t23: F, t2695: F, t11674: F, t11678: F, t15102: F, t15106: F, t15121: F, t18940: F, t2255: F, t2276: F, t2308: F, t342: F, t346: F, t4355: F, t4358: F, t5583: F, t6007: F, t6021: F, t7099: F, t783: F, t7881: F, t8163: F, t8177: F, t8180: F, t8184: F, t8189: F, t8208: F) -> F {
    let t21628 = t4359 * t7344;
    let t21633 = t2695 * t23;
    let t21648 = F::new(6.0) * t5583 * t6007 * t7881 * t342 - F::new(9.0) * t18940 * t4355 - F::cast_from(0.03592270203076383_f64) * t15102 - F::cast_from(0.03592270203076383_f64) * t15106 + F::new(18.0) * t4358 * t21628 + F::cast_from(0.012203831437512505_f64) * t11674 - F::cast_from(0.020146007452401596_f64) * t11678 + F::new(18.0) * t21633 * t2276 - F::new(2.0) * t346 * t2308 * t2255 * t783 - F::new(2.0) * t346 * t6021 * t7099 - F::cast_from(0.15965645347006147_f64) * t15121 - F::cast_from(0.01197423401025461_f64) * t8163 - t8177 - F::cast_from(4.569219094474146e-06_f64) * t8180 - t8184 + F::cast_from(0.05987117005127304_f64) * t8189 + F::cast_from(0.19513566535229734_f64) * t8208;
    t21648
}
