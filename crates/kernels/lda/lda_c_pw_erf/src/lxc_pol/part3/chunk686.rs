//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 686/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk686<F: Float>(t3827: F, t3831: F, t3836: F, t3840: F, t3843: F, t3845: F, t3849: F, t3853: F, t3857: F, t3862: F, t3866: F, t3871: F, t3875: F, t3877: F, t3879: F, t3882: F, t3886: F) -> F {
    let t4211 = -t3827 - t3831 - t3836 - t3840 - t3843 - t3845 - t3849 - t3853 + t3857 + t3862 - t3866 + t3871 + t3875 + t3877 + t3879 + t3882 + t3886;
    t4211
}
