//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 686/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk686(t3827: f64, t3831: f64, t3836: f64, t3840: f64, t3843: f64, t3845: f64, t3849: f64, t3853: f64, t3857: f64, t3862: f64, t3866: f64, t3871: f64, t3875: f64, t3877: f64, t3879: f64, t3882: f64, t3886: f64) -> f64 {
    let t4211 = -t3827 - t3831 - t3836 - t3840 - t3843 - t3845 - t3849 - t3853 + t3857 + t3862 - t3866 + t3871 + t3875 + t3877 + t3879 + t3882 + t3886;
    t4211
}
