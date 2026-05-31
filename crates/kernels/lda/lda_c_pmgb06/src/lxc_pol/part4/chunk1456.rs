//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1456/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1456<F: Float>(t1227: F, t2703: F, t38: F, t4394: F, t776: F, t342: F, t6979: F, t11376: F, t11379: F, t11390: F, t11370: F, t11373: F, t11382: F, t11388: F, t11393: F, t1282: F, t5980: F, t63: F, t6996: F) -> (F, F, F, F, F, F, F) {
    let t18671 = F::cast_from(17.53815_f64) * t38 * t2703 * t1227;
    let t18674 = F::cast_from(11.6921_f64) * t38 * t776 * t4394;
    let t18677 = F::cast_from(11.6921_f64) * t38 * t6979 * t342;
    let t18684 = F::cast_from(0.9743416666666667_f64) * t11376;
    let t18685 = F::cast_from(1.2991222222222223_f64) * t11379;
    let t18688 = F::cast_from(3.031285185185185_f64) * t11390;
    let t18690 = F::cast_from(5.87616_f64) * t63 * t6996 * t1227 - t18671 + t18674 + t18677 + F::cast_from(11.75232_f64) * t63 * t1282 * t5980 * t342 + F::cast_from(1.46904_f64) * t11370 - F::cast_from(1.95872_f64) * t11373 + t18684 + t18685 + t11382 / F::cast_from(3.0_f64) + F::cast_from(4.570346666666667_f64) * t11388 + t18688 + F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t11393;
    (t18671, t18674, t18677, t18684, t18685, t18688, t18690)
}
