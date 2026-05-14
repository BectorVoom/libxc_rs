//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 783/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk783<F: Float>(t108: F, t2268: F, t2274: F, t2329: F, t2337: F, t406: F, t408: F, t659: F, t661: F, t7354: F, t7360: F, t7365: F, t7370: F, t267: F, t3439: F, t6225: F, t7496: F, t7497: F, t7499: F, t7500: F, t7501: F, t7502: F, t7503: F, t7504: F, t7505: F, t7507: F, t7509: F, t7511: F, t7512: F, t7517: F, t7518: F) -> (F, F) {
    let t8025 = (40.0 / 27.0 * t406 * t7354 + 20.0 / 3.0 * t2268 * t2329 + 4.0 / 3.0 * t659 * t7360 + 40.0 / 27.0 * t408 * t7365 + 20.0 / 3.0 * t2274 * t2337 + 4.0 / 3.0 * t661 * t7370) * t108;
    let t8028 = t7496 - t7497 + 2.0 / 3.0 * t6225 + t7499 - t8025 * t267 / 15.0 - t7500 - t7501 + t7502 + t7503 - t7504 - t7505 - t7507 + t7509 + t7511 + t3439 + t7512 - t7517 + t7518;
    (t8025, t8028)
}
