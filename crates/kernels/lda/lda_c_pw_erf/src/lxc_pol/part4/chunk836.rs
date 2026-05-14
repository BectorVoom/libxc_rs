//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 836/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk836<F: Float>(t2011: F, t2146: F, t2014: F, t2018: F, t2419: F, t549: F, t1319: F, t1318: F, t2007: F, t4738: F, t6217: F, t6219: F, t6222: F, t6224: F, t6225: F, t6228: F, t6232: F, t6235: F, t6238: F, t6241: F, t6246: F, t6248: F) -> (F, F, F, F, F, F, F, F) {
    let t6250 = 8.0 / 45.0 * t2146 * t2011;
    let t6252 = 16.0 / 45.0 * t2146 * t2014;
    let t6254 = 8.0 / 27.0 * t2146 * t2018;
    let t6255 = t2419 * t549;
    let t6256 = t1319 * t6255;
    let t6258 = 8.0 / 45.0 * t1318 * t6256;
    let t6260 = 16.0 / 45.0 * t4738 * t2007;
    let t6261 = t6217 + t6219 + t6222 - t6224 + 2.0 / 9.0 * t6225 + t6228 + t6232 - t6235 - t6238 + t6241 + t6246 + t6248 - t6250 - t6252 + t6254 + t6258 + t6260;
    (t6250, t6252, t6254, t6255, t6256, t6258, t6260, t6261)
}
