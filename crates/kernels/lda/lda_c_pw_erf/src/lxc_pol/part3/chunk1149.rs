//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1149/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1149<F: Float>(t13080: F, t4689: F, t571: F, t1124: F, t1484: F, t219: F, t4676: F, t494: F, t542: F, t3965: F, t4490: F, t505: F) -> (F, F, F) {
    let t13452 = t571 * t13080 * t4689;
    let t13453 = F::new(16.0) / F::new(9.0) * t13452;
    let t13455 = t1124 * t1484 * t219;
    let t13457 = t571 * t13455 * t4676;
    let t13458 = F::new(40.0) / F::new(27.0) * t13457;
    let t13459 = t494 * t542;
    let t13463 = F::new(32.0) / F::new(15.0) * t3965 * t4490 * t505 * t13459;
    (t13453, t13458, t13463)
}
