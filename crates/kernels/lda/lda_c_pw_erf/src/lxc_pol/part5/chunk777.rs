//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 777/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk777<F: Float>(t2610: F, t411: F, t2594: F, t1832: F, t756: F, t133: F, t7145: F, t7142: F, t1870: F, t1871: F, t3322: F, t5609: F, t5651: F, t5660: F, t5663: F, t7128: F, t7155: F, t7160: F, t7163: F) -> (F, F, F, F, F, F) {
    let t7191 = t2610 * t411;
    let t7195 = t2594 * t411;
    let t7199 = t756 * t1832;
    let t7203 = t133 * t7145;
    let t7205 = t133 * t7142;
    let t7210 = -t7128 + F::new(5.172765) * t1870 * t1871 * t7191 - F::new(20.69106) * t1870 * t5651 * t7195 + F::new(10.34553) * t1870 * t1871 * t7199 + F::new(0.5747516666666667) * t7203 - F::new(1.724255) * t7205 - F::new(1.724255) * t133 * t7155 - F::new(1.532671111111111) * t5660 + t5663 - t5609 - t7160 + t7163 - t3322;
    (t7191, t7195, t7199, t7203, t7205, t7210)
}
