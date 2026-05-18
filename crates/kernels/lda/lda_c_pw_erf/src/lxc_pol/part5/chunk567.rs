//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 567/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk567<F: Float>(t1064: F, t358: F, t1039: F, t339: F, t344: F, t1191: F, t169: F, t301: F, t678: F, t119: F, t411: F, t473: F) -> (F, F, F, F, F, F, F, F) {
    let t3179 = t1064 * t358;
    let t3180 = F::new(60.0) * t3179;
    let t3181 = t339 * t1039;
    let t3182 = F::new(24.0) * t3181;
    let t3183 = t344 * t1039;
    let t3184 = F::new(24.0) * t3183;
    let t3203 = t169 * t1191 * t678 * t301;
    let t3216 = t119 * t473 * t411;
    (t3179, t3180, t3181, t3182, t3183, t3184, t3203, t3216)
}
