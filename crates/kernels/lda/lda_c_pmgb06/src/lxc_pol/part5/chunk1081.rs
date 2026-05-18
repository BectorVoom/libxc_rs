//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1081/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1081<F: Float>(t19989: F, t5083: F, t5084: F, t10693: F, t10696: F, t12461: F, t12463: F, t19985: F, t19986: F, t19987: F, t19988: F, t19992: F, t19995: F, t19998: F) -> (F, F) {
    let t20001 = t5083 * t5084 * t19989 / F::new(9.0);
    let t20003 = -t19985 - t12461 - t12463 + t19986 + t19987 + t19988 + t19992 + t19995 - t19998 - t20001 + F::new(0.0011033703703703704) * t10693 + t10696;
    (t20001, t20003)
}
