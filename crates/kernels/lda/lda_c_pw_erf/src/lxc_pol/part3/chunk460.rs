//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 460/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk460<F: Float>(t1799: F, t40: F, t1042: F, t1046: F, t1798: F, t85: F, t140: F, t1729: F) -> (F, F, F, F, F) {
    let t1800 = t40 * t1799;
    let t1801 = F::new(4.0) * t1042;
    let t1802 = F::new(4.0) * t1046;
    let t1804 = t1798 * t85;
    let t1805 = F::cast_from(0.019751789702565206_f64) * t1804;
    let t1808 = t1729 * t140;
    (t1800, t1801, t1802, t1805, t1808)
}
