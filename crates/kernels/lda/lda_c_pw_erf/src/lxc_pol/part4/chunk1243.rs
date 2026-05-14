//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1243/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1243<F: Float>(t4763: F, t4785: F, t2146: F, t4788: F, t14089: F, t18436: F, t18439: F, t18442: F, t18445: F, t18447: F, t18450: F, t18452: F, t18454: F, t18459: F, t18463: F, t18467: F, t18469: F, t18471: F) -> (F, F, F, F) {
    let t18473 = 64.0 / 45.0 * t4763 * t4785;
    let t18474 = t2146 * t4788;
    let t18475 = 32.0 / 135.0 * t18474;
    let t18476 = 32.0 / 405.0 * t14089;
    let t18477 = -t18436 + t18439 - t18442 - t18445 - t18447 + t18450 + t18452 + t18454 + t18459 + t18463 - t18467 + t18469 + t18471 - t18473 - t18475 + t18476;
    (t18473, t18475, t18476, t18477)
}
