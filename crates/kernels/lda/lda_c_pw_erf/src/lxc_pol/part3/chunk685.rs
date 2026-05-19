//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 685/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk685<F: Float>(t1638: F, t4207: F, t3796: F, t3801: F, t3805: F, t3810: F, t3814: F, t3816: F, t3821: F, t3823: F, t4185: F, t4188: F, t4190: F, t4193: F, t4198: F, t4201: F, t4202: F, t4206: F) -> (F, F) {
    let t4209 = F::cast_from(0.011181742741110338_f64) * t1638 * t4207;
    let t4210 = -t3796 - t3801 - t4185 + F::cast_from(0.3246312408709453_f64) * t4188 + F::cast_from(0.6492624817418906_f64) * t4190 + F::cast_from(0.03354522822333102_f64) * t4193 + t4198 + t4201 + F::cast_from(0.21642082724729686_f64) * t4202 + t4206 - t4209 - t3805 + t3810 + t3814 + t3816 + t3821 - t3823;
    (t4209, t4210)
}
