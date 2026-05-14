//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1142/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1142<F: Float>(t2029: F, t5509: F, t5508: F, t1586: F, t2804: F, t33162: F, t32967: F, t32970: F, t32975: F, t32978: F, t32982: F, t32987: F, t33180: F, t33188: F, t33258: F, t9725: F, t9728: F, t9733: F, t9748: F) -> (F, F, F, F) {
    let t33261 = t2029 * t5509;
    let t33262 = t5508 * t33261;
    let t33263 = t1586 * t33262;
    let t33270 = t2804 * t33162;
    let t33272 = 0.10416666666666666667e-1 * t9733 * t9748 + 0.10416666666666666667e-1 * t9733 * t9728 - 0.34822083333333333332e-2 * t32967 + 0.23214722222222222222e-2 * t32970 - 0.17411041666666666666e-2 * t32975 - 0.38691203703703703703e-3 * t32978 + 0.34822083333333333332e-2 * t32982 - 0.23214722222222222222e-2 * t32987 + 0.40208333333333333334e-2 * t33258 * t9728 - 0.10416666666666666667e-1 * t2804 * t33263 - 0.10416666666666666667e-1 * t2804 * t33180 + 0.20104166666666666667e-2 * t9725 * t33188 + 0.34722222222222222222e-2 * t33270;
    (t33262, t33263, t33270, t33272)
}
