//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1456/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1456<F: Float>(t1227: F, t2703: F, t38: F, t4394: F, t776: F, t342: F, t6979: F, t11376: F, t11379: F, t11390: F, t11370: F, t11373: F, t11382: F, t11388: F, t11393: F, t1282: F, t5980: F, t63: F, t6996: F) -> (F, F, F, F, F, F, F) {
    let t18671 = F::new(17.53815) * t38 * t2703 * t1227;
    let t18674 = F::new(11.6921) * t38 * t776 * t4394;
    let t18677 = F::new(11.6921) * t38 * t6979 * t342;
    let t18684 = F::new(0.9743416666666667) * t11376;
    let t18685 = F::new(1.2991222222222223) * t11379;
    let t18688 = F::new(3.031285185185185) * t11390;
    let t18690 = F::new(5.87616) * t63 * t6996 * t1227 - t18671 + t18674 + t18677 + F::new(11.75232) * t63 * t1282 * t5980 * t342 + F::new(1.46904) * t11370 - F::new(1.95872) * t11373 + t18684 + t18685 + t11382 / F::new(3.0) + F::new(4.570346666666667) * t11388 + t18688 + F::new(28.0) / F::new(27.0) * t11393;
    (t18671, t18674, t18677, t18684, t18685, t18688, t18690)
}
