//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1067/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1067<F: Float>(t2540: F, t3727: F, t1446: F, t6696: F, t15501: F, t15506: F, t15508: F, t15513: F, t15514: F, t15515: F, t15517: F, t15519: F, t15522: F, t15524: F, t15526: F, t15531: F, t15533: F, t15535: F) -> (F, F, F) {
    let t15537 = 4.0 / 45.0 * t3727 * t2540;
    let t15538 = t1446 * t6696;
    let t15539 = 16.0 / 135.0 * t15538;
    let t15540 = 8.0 / 3.0 * t15501 + t15506 - t15508 + t15513 + t15514 - t15515 + t15517 + t15519 - t15522 + t15524 + t15526 + t15531 - t15533 - t15535 + t15537 + t15539;
    (t15537, t15539, t15540)
}
