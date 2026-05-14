//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 725/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk725<F: Float>(t56: F, t7306: F, t38: F, t370: F, t2448: F, t780: F, t64: F, t35: F, t1282: F, t7277: F, t3505: F, t3513: F, t3515: F, t3517: F, t3521: F, t3523: F, t3525: F, t360: F, t63: F, t7278: F, t7283: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7307 = t56 * t7306;
    let t7309 = 2.923025 * t38 * t7307;
    let t7310 = t370 * t7306;
    let t7313 = t780 * t2448;
    let t7317 = t64 * t7306;
    let t7318 = t35 * t7317;
    let t7321 = t1282 * t7277;
    let t7322 = t35 * t7321;
    let t7325 = -t3505 + t3513 - 29.3808 * t63 * t7278 - t7283 - t7309 - 1.46904 * t63 * t7310 + 9.0 / 2.0 * t360 * t35 * t7313 - t3515 - t3517 - t3521 - t3523 + t3525 - t360 * t7318 / 2.0 - 6.0 * t360 * t7322;
    (t7307, t7309, t7310, t7313, t7317, t7318, t7321, t7322, t7325)
}
