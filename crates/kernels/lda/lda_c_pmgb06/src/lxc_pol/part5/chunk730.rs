//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 730/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk730<F: Float>(t283: F, t3672: F, t3678: F, t4518: F, t4520: F, t4522: F, t4525: F, t4527: F, t6038: F, t6071: F, t6072: F, t6074: F, t7402: F, t285: F, t248: F, t3700: F, t3707: F, t3713: F, t3719: F, t3727: F, t3731: F, t3736: F, t3744: F, t3762: F, t4532: F, t4534: F, t6079: F) -> (F, F, F) {
    let t7410 = -1.7544670867903938 * t6038 - 24.0 * t4518 + 36.0 * t4520 + 60.0 * t4522 + 0.0197516734986138 * t7402 * t283 + 3.0 * t6071 - 12.0 * t6072 - 12.0 * t6074 + 3.0 * t4525 + 96.0 * t4527 + t3672 - t3678;
    let t7414 = t7402 * t285;
    let t7416 = t3700 - 0.0005493434191801964 * t6079 + 0.0007324578922402618 * t4532 - 24.0 * t4534 + t248 * t7414 - t3707 + t3713 + t3719 - t3727 + t3731 - t3736 - t3744 - t3762;
    (t7410, t7414, t7416)
}
