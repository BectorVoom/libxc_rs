//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1186/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1186<F: Float>(t13975: F, t1284: F, t5175: F, t10454: F, t10456: F, t10465: F, t13952: F, t13956: F, t13958: F, t13961: F, t13965: F, t13969: F, t13972: F, t13974: F) -> (F, F, F, F, F, F) {
    let t13976 = F::new(4.0) / F::new(3.0) * t13975;
    let t13977 = t1284 * t5175;
    let t13978 = F::new(4.0) / F::new(3.0) * t13977;
    let t13979 = F::new(8.0) / F::new(45.0) * t10454;
    let t13980 = F::new(16.0) / F::new(45.0) * t10456;
    let t13981 = F::new(32.0) / F::new(135.0) * t10465;
    let t13982 = -t13952 + t13956 - t13958 + t13961 + t13965 - t13969 + t13972 - t13974 + t13976 + t13978 - t13979 - t13980 - t13981;
    (t13976, t13978, t13979, t13980, t13981, t13982)
}
