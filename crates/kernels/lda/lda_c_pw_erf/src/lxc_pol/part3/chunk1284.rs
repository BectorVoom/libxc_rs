//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1284/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1284<F: Float>(t12812: F, t12815: F, t12818: F, t12821: F, t12824: F, t12829: F, t12832: F, t12836: F, t12839: F, t12842: F, t12844: F, t12846: F, t12848: F) -> F {
    let t15050 = -t12812 + t12815 + t12818 + t12821 - t12824 - t12829 + t12832 - t12836 + t12839 + t12842 + t12844 - t12846 - t12848;
    t15050
}
