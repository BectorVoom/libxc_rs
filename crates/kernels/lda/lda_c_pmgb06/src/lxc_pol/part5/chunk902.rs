//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 902/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk902<F: Float>(t2778: F, t415: F, t2791: F, t399: F, t10524: F, t117: F, t84: F, t118: F, t3993: F, t391: F, t1329: F, t1347: F) -> (F, F, F, F, F, F) {
    let t10847 = F::new(0.0004746123948660562) * t2778 * t415;
    let t10848 = t399 * t2791;
    let t10852 = F::new(0.031505407223141116) * t84 * t10524 * t117;
    let t10857 = t3993 * t118;
    let t10860 = F::new(0.12602162889256446) * t391 * t2791;
    let t10861 = t1329 * t1347;
    (t10847, t10848, t10852, t10857, t10860, t10861)
}
