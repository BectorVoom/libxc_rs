//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 820/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk820<F: Float>(t6011: F, t85: F, t411: F, t770: F, t2765: F, t1734: F, t2591: F, t1729: F, t454: F, t776: F, t2363: F, t299: F, t169: F, t242: F, t2852: F, t4711: F, t4712: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6012 = t6011 * t85;
    let t6013 = 0.019751789702565206 * t6012;
    let t6015 = t770 * t411;
    let t6016 = t2765 * t6015;
    let t6019 = t2591 * t1734;
    let t6025 = t1729 * t776 * t454;
    let t6035 = t299 * t2363;
    let t6037 = t169 * t6035 * t242;
    let t6039 = -t4711 + t4712 + t2852;
    (t6012, t6013, t6015, t6016, t6019, t6025, t6035, t6037, t6039)
}
