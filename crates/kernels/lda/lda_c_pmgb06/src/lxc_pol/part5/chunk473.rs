//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 473/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk473<F: Float>(t1133: F, t1138: F, t1141: F, t1145: F, t1151: F, t123: F, t1316: F, t1317: F, t1324: F, t1772: F, t1775: F, t2164: F, t2276: F, t2306: F, t2308: F, t2312: F, t2365: F, t312: F, t315: F, t317: F, t329: F, t346: F, t61: F, t790: F) -> F {
    let t2367 = -t1772 - t1775 - F::new(0.054045904796391424) * t1133 + t1138 - F::new(0.0002905674151788692) * t1141 - t1145 + t1151 + F::new(3.0) * t329 * t2276 + t2306 * t312 - t346 * t2308 * t1324 + F::new(3.0) * t1316 * t2312 + F::new(3.0) * t1316 * t790 * t1317 + F::new(0.020267214298646783) * t123 * t315 * t2164 * t317 + t2365 * t61;
    t2367
}
