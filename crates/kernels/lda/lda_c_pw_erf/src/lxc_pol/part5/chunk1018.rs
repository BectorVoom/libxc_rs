//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1018/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1018<F: Float>(t6209: F, t6592: F, t2104: F, t7838: F, t16935: F, t16949: F, t16952: F, t21294: F, t21296: F, t21303: F, t21305: F, t21307: F, t21309: F, t21311: F, t21313: F, t16955: F) -> (F, F, F, F, F, F, F) {
    let t21315 = 4.0 / 5.0 * t6209 * t6592;
    let t21317 = 4.0 / 15.0 * t2104 * t7838;
    let t21318 = 8.0 / 27.0 * t16935;
    let t21319 = 16.0 / 45.0 * t16949;
    let t21320 = 8.0 / 45.0 * t16952;
    let t21321 = -t21294 + t21296 + t21303 - t21305 + t21307 - t21309 + t21311 + t21313 + t21315 + t21317 - t21318 + t21319 - t21320;
    let t21322 = 16.0 / 45.0 * t16955;
    (t21315, t21317, t21318, t21319, t21320, t21321, t21322)
}
