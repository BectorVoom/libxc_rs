//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1042/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1042<F: Float>(t4738: F, t5378: F, t511: F, t6306: F, t1529: F, t2425: F, t1446: F, t6682: F, t518: F, t6670: F, t1397: F, t6601: F) -> (F, F, F, F, F, F) {
    let t18630 = t4738 * t5378;
    let t18642 = t511 * t6306;
    let t18655 = t2425 * t1529;
    let t18673 = t1446 * t6682;
    let t18681 = t6670 * t518;
    let t18695 = t6601 * t1397;
    (t18630, t18642, t18655, t18673, t18681, t18695)
}
