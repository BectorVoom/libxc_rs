//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1072/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1072<F: Float>(t22301: F, t2171: F, t6699: F, t22263: F, t22266: F, t22268: F, t22272: F, t22276: F, t22280: F, t22284: F, t22288: F, t22292: F, t22296: F, t22299: F, t6696: F, t1475: F, t571: F, t7608: F) -> (F, F, F, F, F) {
    let t22302 = 8.0 / 135.0 * t22301;
    let t22303 = t2171 * t6699;
    let t22304 = 8.0 / 27.0 * t22303;
    let t22305 = t22263 + t22266 - t22268 + t22272 - t22276 - t22280 + t22284 + t22288 + t22292 + t22296 + t22299 + t22302 + t22304;
    let t22306 = t2171 * t6696;
    let t22307 = 8.0 / 45.0 * t22306;
    let t22309 = t571 * t1475 * t7608;
    (t22302, t22304, t22305, t22307, t22309)
}
