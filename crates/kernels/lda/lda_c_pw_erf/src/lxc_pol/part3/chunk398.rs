//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 398/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk398<F: Float>(t549: F, t581: F, t593: F, t1466: F, t1318: F, t518: F, t564: F) -> (F, F, F, F) {
    let t1468 = t581 * t549 * t593;
    let t1469 = t1466 * t1468;
    let t1471 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t1469;
    let t1472 = t564 * t518;
    (t1468, t1469, t1471, t1472)
}
