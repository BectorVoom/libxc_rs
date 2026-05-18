//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 931/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk931<F: Float>(t1369: F, t3309: F, t1372: F, t1375: F, t186: F, t740: F, t934: F, t117: F, t123: F, t1650: F, t2779: F, t1147: F, t398: F) -> (F, F, F, F, F, F) {
    let t10770 = t1369 * t3309;
    let t10773 = F::new(0.38474813732852775) * t1372 * t3309;
    let t10777 = F::new(0.019878653761973935) * t1375 * t934 * t740 * t186;
    let t10790 = t123 * t740 * t1650 * t117;
    let t10792 = F::new(0.7561297733553868) * t2779;
    let t10793 = t1147 * t398;
    (t10770, t10773, t10777, t10790, t10792, t10793)
}
