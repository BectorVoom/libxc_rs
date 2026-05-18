//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1240/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1240<F: Float>(t1325: F, t34: F, t4956: F, t6944: F, t10557: F, t519: F, t7624: F, t1449: F, t7620: F, t2171: F, t6699: F, t22263: F, t22266: F, t22268: F, t22272: F, t22276: F, t22280: F, t22284: F, t22288: F, t22292: F) -> (F, F, F, F, F) {
    let t22296 = F::new(8.0) / F::new(5.0) * t1325 * t4956 * t6944 * t34;
    let t22298 = t519 * t10557 * t7624;
    let t22299 = F::new(64.0) / F::new(243.0) * t22298;
    let t22301 = t519 * t1449 * t7620;
    let t22302 = F::new(8.0) / F::new(135.0) * t22301;
    let t22303 = t2171 * t6699;
    let t22304 = F::new(8.0) / F::new(27.0) * t22303;
    let t22305 = t22263 + t22266 - t22268 + t22272 - t22276 - t22280 + t22284 + t22288 + t22292 + t22296 + t22299 + t22302 + t22304;
    (t22296, t22299, t22302, t22304, t22305)
}
