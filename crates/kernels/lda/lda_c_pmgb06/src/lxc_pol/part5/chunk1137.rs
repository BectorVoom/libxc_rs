//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1137/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1137<F: Float>(t2002: F, t6241: F, t6245: F, t20627: F, t20632: F, t20636: F, t20641: F, t20643: F, t20646: F, t20648: F, t20651: F, t20654: F, t20656: F) -> (F, F, F) {
    let t20658 = t2002 * t6241 / F::cast_from(15.0_f64);
    let t20660 = t2002 * t6245 / F::cast_from(15.0_f64);
    let t20661 = t20627 + t20632 - t20636 + t20641 - t20643 - t20646 + t20648 + t20651 + t20654 + t20656 + t20658 + t20660;
    (t20658, t20660, t20661)
}
