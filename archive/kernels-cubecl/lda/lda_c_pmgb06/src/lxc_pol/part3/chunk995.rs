//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 995/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk995<F: Float>(t1431: F, t5187: F, t1441: F, t1447: F, t5176: F, t1989: F, t3226: F, t1499: F, t2090: F, t3146: F, t853: F, t11813: F, t11815: F, t11816: F, t11820: F, t11823: F, t11825: F) -> (F, F, F, F, F, F, F) {
    let t11827 = t5187 * t1431 / F::cast_from(15.0_f64);
    let t11829 = t5187 * t1441 / F::cast_from(9.0_f64);
    let t11830 = t1447 * t5176;
    let t11831 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t11830;
    let t11832 = t3226 * t1989;
    let t11833 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t11832;
    let t11835 = t1499 * t2090 / F::cast_from(10.0_f64);
    let t11837 = t3146 * t853 / F::cast_from(30.0_f64);
    let t11838 = -F::cast_from(0.013506172839506173_f64) * t11813 + t11815 - t11816 + t11820 + t11823 + t11825 + t11827 + t11829 + t11831 + t11833 - t11835 - t11837;
    (t11827, t11829, t11831, t11833, t11835, t11837, t11838)
}
