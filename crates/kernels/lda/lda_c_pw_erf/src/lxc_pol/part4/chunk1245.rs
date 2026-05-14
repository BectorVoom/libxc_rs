//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1245/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1245<F: Float>(t1318: F, t3899: F, t6953: F, t14093: F, t14095: F, t14098: F, t14100: F, t14103: F, t14105: F, t18484: F, t18486: F, t18488: F, t18491: F, t18493: F, t18495: F, t18497: F, t18499: F, t18501: F, t18503: F) -> (F, F) {
    let t18505 = t1318 * t3899 * t6953;
    let t18506 = 32.0 / 45.0 * t18505;
    let t18507 = 0.21642082724729686 * t14093 + 0.8656833089891874 * t14095 + 0.04472697096444135 * t14098 + 0.6492624817418906 * t14100 + 0.06709045644666203 * t14103 - 0.022363485482220676 * t14105 + t18484 + t18486 + t18488 + t18491 + t18493 + t18495 + t18497 + t18499 + t18501 + t18503 - t18506;
    (t18506, t18507)
}
