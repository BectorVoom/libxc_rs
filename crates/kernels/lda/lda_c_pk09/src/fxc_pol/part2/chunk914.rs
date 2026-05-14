//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 914/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk914<F: Float>(t1675: F, t2759: F, t1672: F, t2755: F, t11101: F, t1797: F, t1800: F, t2901: F, t6230: F, t452: F, t1947: F, t2902: F, t2042: F, t10: F, t1729: F, t549: F) -> (F, F, F, F, F, F) {
    let t11262 = t2759 * t1675;
    let t11264 = t2755 * t1672;
    let t11270 = t1797 * t11101;
    let t11271 = t11270 * t1800;
    let t11273 = t2901 * t6230;
    let t11274 = t11273 * t452;
    let t11277 = t2902 * t1947;
    let t11278 = t11277 * t2042;
    let t11282 = t1729 * t10;
    let t11283 = t549 * t11282;
    (t11262, t11264, t11271, t11274, t11278, t11283)
}
