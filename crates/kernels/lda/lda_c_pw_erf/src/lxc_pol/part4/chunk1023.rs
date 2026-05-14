//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1023/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1023<F: Float>(t13440: F, t4620: F, t519: F, t4900: F, t581: F, t4842: F, t571: F, t13080: F, t4689: F, t1124: F, t1484: F, t219: F, t4676: F, t494: F, t542: F, t3576: F, t822: F) -> (F, F, F, F, F, F, F, F) {
    let t13442 = t519 * t13440 * t4620;
    let t13444 = t4900 * t581;
    let t13446 = t571 * t13444 * t4842;
    let t13452 = t571 * t13080 * t4689;
    let t13455 = t1124 * t1484 * t219;
    let t13457 = t571 * t13455 * t4676;
    let t13459 = t494 * t542;
    let t13464 = t822 * t3576;
    (t13442, t13444, t13446, t13452, t13455, t13457, t13459, t13464)
}
