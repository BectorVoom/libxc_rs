//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1188/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1188<F: Float>(t18728: F, t18731: F, t18734: F, t18747: F, t18749: F, t1282: F, t18718: F, t18721: F, t18725: F, t18745: F, t18752: F, t342: F, t63: F, t7306: F) -> (F, F, F, F, F, F) {
    let t21461 = F::cast_from(1.9486833333333333_f64) * t18728;
    let t21462 = F::cast_from(1.4615125_f64) * t18731;
    let t21463 = F::cast_from(0.9743416666666667_f64) * t18734;
    let t21465 = F::cast_from(3.8973666666666666_f64) * t18747;
    let t21466 = F::cast_from(1.9486833333333333_f64) * t18749;
    let t21468 = F::cast_from(5.87616_f64) * t63 * t1282 * t7306 * t342 + F::cast_from(5.87616_f64) * t18718 + F::cast_from(2.20356_f64) * t18721 - F::cast_from(1.46904_f64) * t18725 - t21461 + t21462 + t21463 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18745 + t21465 - t21466 + t18752 / F::cast_from(2.0_f64);
    (t21461, t21462, t21463, t21465, t21466, t21468)
}
