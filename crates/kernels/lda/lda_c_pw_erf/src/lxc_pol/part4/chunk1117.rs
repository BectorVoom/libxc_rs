//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1117/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1117<F: Float>(t1349: F, t1943: F, t1953: F, t1371: F, t15752: F, t16100: F, t16105: F, t16308: F, t16311: F, t16314: F, t16325: F, t16327: F, t16332: F, t16336: F, t16338: F, t25: F, t3587: F, t589: F) -> (F, F) {
    let t16341 = t1953 * t1349 * t1943;
    let t16343 = -0.047988888888888886 * t16308 - 0.023994444444444443 * t16311 - 0.03999074074074074 * t16314 - 0.0022222222222222222 * t25 * t1371 * t16100 - 0.002962962962962963 * t25 * t3587 * t16105 - 0.04 * t25 * t589 * t15752 - 0.047988888888888886 * t16325 + 0.015996296296296297 * t16327 + 0.14396666666666666 * t16332 + 0.07198333333333333 * t16336 - 0.3519185185185185 * t16338 + 0.09597777777777777 * t16341;
    (t16341, t16343)
}
