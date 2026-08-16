//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1235/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1235<F: Float>(t18257: F, t18259: F, t20317: F, t20319: F, t20321: F, t20323: F, t20324: F, t20325: F, t20328: F, t20330: F, t20332: F, t20334: F, t20337: F, t20338: F, t20340: F, t20343: F, t20346: F, t20353: F, t20355: F, t20359: F, t20361: F, t20364: F, t20367: F) -> (F, F) {
    let t21987 = -t20317 - t20319 - t20321 + t20323 + t20324 + t20325 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18257 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t18259 - t20328 - t20330 - t20332;
    let t21988 = -t20334 - t20337 + t20338 + t20340 - t20343 + t20346 + t20353 - t20355 + t20359 + t20361 + t20364 + t20367;
    (t21987, t21988)
}
