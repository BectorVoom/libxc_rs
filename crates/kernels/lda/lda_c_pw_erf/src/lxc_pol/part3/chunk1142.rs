//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1142/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1142<F: Float>(t13375: F, t1620: F, t838: F, t1931: F, t610: F, t230: F, t4714: F, t4521: F, t833: F, t3610: F, t4506: F, t211: F, t4567: F, t4575: F) -> (F, F, F, F, F, F, F) {
    let t13376 = F::new(8.0) / F::new(27.0) * t13375;
    let t13377 = t838 * t1620;
    let t13379 = t1931 * t610;
    let t13380 = F::new(8.0) * t13379;
    let t13381 = t4714 * t230;
    let t13384 = t4521 * t833;
    let t13387 = F::new(4.0) / F::new(9.0) * t4506 * t13384 * t3610;
    let t13389 = t211 * t4567 * t4575;
    (t13376, t13377, t13380, t13381, t13384, t13387, t13389)
}
