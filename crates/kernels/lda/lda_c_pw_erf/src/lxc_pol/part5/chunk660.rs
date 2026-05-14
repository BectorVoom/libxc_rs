//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 660/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk660<F: Float>(t348: F, t6330: F, t1326: F, t519: F, t1245: F, t2329: F, t1991: F, t3682: F, t3706: F, t4583: F, t5806: F, t5837: F, t6312: F, t6313: F, t6316: F, t6317: F, t6318: F, t6319: F, t6320: F, t6321: F, t6325: F, t6329: F) -> (F, F, F, F, F, F, F, F) {
    let t6331 = t6330 * t348;
    let t6332 = t1326 * t6331;
    let t6334 = 8.0 / 45.0 * t519 * t6332;
    let t6335 = t1245 * t2329;
    let t6336 = t6335 * t348;
    let t6337 = t1991 * t6336;
    let t6339 = 4.0 / 27.0 * t519 * t6337;
    let t6340 = -t6312 + t6313 + 4.0 / 135.0 * t5806 + 2.0 / 135.0 * t3682 - t3706 - t5837 - t6316 - t6317 + t6318 - t6319 + t6320 - t6321 + t4583 + t6325 - t6329 - t6334 + t6339;
    (t6331, t6332, t6334, t6335, t6336, t6337, t6339, t6340)
}
