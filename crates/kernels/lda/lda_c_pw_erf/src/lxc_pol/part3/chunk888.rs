//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 888/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk888<F: Float>(t9178: F, t9180: F, t9181: F, t9186: F, t9190: F, t9192: F, t9195: F, t9197: F, t9199: F, t9201: F, t9203: F, t9206: F, t9207: F, t9211: F, t9215: F, t164: F, t4422: F) -> (F, F) {
    let t11619 = t9178 - t9180 - 0.00035595929614954216 * t9181 - 0.031505407223141116 * t9186 - 0.07184540406152766 * t9190 - 0.5670973300165402 * t9192 - t9195 + 0.031505407223141116 * t9197 + 0.1890324433388467 * t9199 - 0.1890324433388467 * t9201 + 0.2835486650082701 * t9203 + t9206 - 0.09451622166942335 * t9207 + 0.2634331482256014 * t9211 + 0.008980675507690957 * t9215;
    let t11620 = t4422 * t164;
    (t11619, t11620)
}
