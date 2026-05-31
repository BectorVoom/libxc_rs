//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 820/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk820<F: Float>(t2093: F, t2623: F, t166: F, t161: F, t2625: F, t831: F, t2592: F, t824: F, t2631: F, t802: F, t6611: F, t6614: F) -> (F, F, F, F, F, F, F, F) {
    let t7747 = t2093 * t2623;
    let t7748 = t166 * t7747;
    let t7750 = t161 * t7748 / F::cast_from(10.0_f64);
    let t7752 = t831 * t2625 / F::cast_from(10.0_f64);
    let t7754 = t2592 * t824 / F::cast_from(10.0_f64);
    let t7756 = t802 * t2631 / F::cast_from(5.0_f64);
    let t7758 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t6611;
    let t7759 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t6614;
    (t7747, t7748, t7750, t7752, t7754, t7756, t7758, t7759)
}
