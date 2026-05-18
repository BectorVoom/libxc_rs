//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1048/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1048<F: Float>(t2777: F, t6945: F, t452: F, t1941: F, t309: F, t454: F, t1947: F, t2919: F, t2042: F, t1240: F, t2923: F, t1905: F) -> (F, F, F, F) {
    let t11400 = t2777 * t6945;
    let t11401 = t11400 * t452;
    let t11403 = t309 * t454 * t1941;
    let t11406 = t2919 * t1947;
    let t11407 = t11406 * t2042;
    let t11411 = t2923 * t1240;
    let t11412 = t1905 * t11411;
    (t11401, t11403, t11407, t11412)
}
