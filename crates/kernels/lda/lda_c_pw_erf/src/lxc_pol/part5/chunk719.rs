//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 719/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk719<F: Float>(t6232: F, t6235: F, t6238: F, t6241: F, t6246: F, t6248: F, t6250: F, t6252: F, t6254: F, t6258: F, t6260: F, t6267: F, t6269: F, t6274: F, t6279: F, t6284: F) -> (F,) {
    let t7239 = t6232 - t6235 - t6238 + t6241 + t6246 + t6248 - t6250 - t6252 + t6254 + t6258 + t6260 + t6267 - t6269 + t6274 + t6279 - t6284;
    (t7239,)
}
