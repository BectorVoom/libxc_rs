//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1271/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1271<F: Float>(t18415: F, t1325: F, t3859: F, t7808: F, t2171: F, t6233: F, t4738: F, t6292: F, t6230: F, t1318: F, t3854: F, t7733: F) -> (F, F, F, F, F, F) {
    let t22821 = F::new(32.0) / F::new(45.0) * t18415;
    let t22823 = t1325 * t3859 * t7808;
    let t22824 = F::new(16.0) / F::new(45.0) * t22823;
    let t22825 = t2171 * t6233;
    let t22826 = F::new(16.0) / F::new(45.0) * t22825;
    let t22827 = t4738 * t6292;
    let t22828 = F::new(32.0) / F::new(45.0) * t22827;
    let t22830 = F::new(16.0) / F::new(15.0) * t4738 * t6230;
    let t22832 = t1318 * t3854 * t7733;
    (t22821, t22824, t22826, t22828, t22830, t22832)
}
