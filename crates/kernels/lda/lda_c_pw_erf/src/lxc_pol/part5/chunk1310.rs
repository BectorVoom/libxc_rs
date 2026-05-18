//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1310/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1310<F: Float>(t21058: F, t21059: F, t21060: F, t21064: F, t21066: F, t21067: F, t21069: F, t21071: F, t21073: F, t21077: F, t21083: F, t21085: F, t21087: F) -> F {
    let t23214 = t21058 + t21059 - t21060 + t21064 - t21066 + t21067 + t21069 - t21071 - t21073 + t21077 + t21083 - t21085 + t21087;
    t23214
}
