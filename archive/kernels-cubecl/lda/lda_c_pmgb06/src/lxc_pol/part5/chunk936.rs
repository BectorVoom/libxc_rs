//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 936/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk936<F: Float>(t154: F, t3092: F, t465: F, t12535: F, t441: F, t5075: F, t1464: F, t1601: F, t2918: F, t518: F, t1554: F, t161: F, t2089: F) -> (F, F, F, F, F, F) {
    let t13027 = t154 * t3092;
    let t13031 = t465 * t3092;
    let t13043 = t5075 * t12535 * t441;
    let t13064 = t1601 * t1464;
    let t13068 = t518 * t2918;
    let t13087 = t161 * t1554 * t2089;
    (t13027, t13031, t13043, t13064, t13068, t13087)
}
