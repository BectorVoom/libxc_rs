//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 838/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk838<F: Float>(t4161: F, t4165: F, t6715: F, t6723: F, t6726: F, t7805: F, t7810: F, t7815: F, t7816: F, t7817: F, t7818: F, t7819: F, t7820: F, t7821: F, t7822: F, t7861: F, t7865: F) -> F {
    let t7985 = -t7805 + t6715 + t6723 + F::cast_from(0.18233333333333332_f64) * t6726 - t7810 - t7815 - t4161 + t4165 + t7816 + t7817 + t7818 + t7819 + t7820 - t7821 + t7822 - t7861 - t7865;
    t7985
}
