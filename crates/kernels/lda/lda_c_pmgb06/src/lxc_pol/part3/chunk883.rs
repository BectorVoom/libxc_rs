//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 883/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk883<F: Float>(t11790: F, t11793: F, t11795: F, t11796: F, t11799: F, t11802: F, t11804: F, t11805: F, t11806: F, t11807: F, t11808: F, t11810: F, t2029: F, t4119: F, t9311: F, t9313: F) -> (F, F, F, F) {
    let t11812 = -t11790 - t11793 - t11795 + 0.09973633333333333 * t11796 + t11799 + t11802 - t11804 - t11805 - t11806 + t11807 - t11808 + 0.001515438175925926 * t11810;
    let t11813 = t2029 * t4119;
    let t11815 = 4.0 / 45.0 * t9311;
    let t11816 = 4.0 / 45.0 * t9313;
    (t11812, t11813, t11815, t11816)
}
