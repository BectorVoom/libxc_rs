//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1115/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1115<F: Float>(t10868: F, t10872: F, t10970: F, t14516: F, t14896: F, t14899: F, t14904: F, t14906: F, t14911: F, t159: F, t18809: F, t19866: F, t20179: F, t20646: F, t23100: F, t23115: F, t279: F, t281: F, t285: F, t4430: F, t5740: F, t5783: F, t6015: F, t6016: F, t6089: F) -> (F,) {
    let t23118 = t14516 - 9.0 * t18809 * t4430 - 6.0 * t5783 * t19866 * t6015 + 9.0 * t6089 * t5740 - 6.0 * t18809 * t6016 - 0.0008717022455366076 * t10868 - t10872 - 0.01197423401025461 * t281 * t20179 * t159 * t285 - 0.01197423401025461 * t20646 - t14896 - 0.03592270203076383 * t14899 - t14904 - 0.0001639671923854359 * t14906 - 1.370765728342244e-05 * t14911 + (t23100 + t23115) * t279 - t10970;
    (t23118,)
}
