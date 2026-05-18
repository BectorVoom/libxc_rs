//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1316/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1316<F: Float>(t12197: F, t12310: F, t12357: F, t21257: F, t21261: F, t21262: F, t21263: F, t21264: F, t21265: F, t21266: F, t21267: F, t21269: F, t21271: F) -> F {
    let t23227 = -t21257 - t21261 + t21262 + t21263 - t21264 + t12197 + t12310 + t21265 - t21266 + t21267 - t21269 + t12357 + t21271;
    t23227
}
