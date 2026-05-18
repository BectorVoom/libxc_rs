//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 655/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk655<F: Float>(t147: F, t740: F, t146: F, t164: F, t2899: F, t1400: F, t187: F, t186: F, t395: F, t184: F, t1403: F, t1410: F) -> (F, F, F, F, F, F, F, F) {
    let t3365 = t740 * t147;
    let t3368 = F::new(0.02962962962962963) * t146 * t3365 * t164;
    let t3380 = F::new(0.11197407407407407) * t2899;
    let t3387 = t1400 * t187;
    let t3389 = t395 * t186;
    let t3391 = F::new(0.0011033703703703704) * t184 * t3389;
    let t3392 = t1403 * t187;
    let t3395 = F::new(4.0) * t1410 * t187;
    (t3365, t3368, t3380, t3387, t3389, t3391, t3392, t3395)
}
