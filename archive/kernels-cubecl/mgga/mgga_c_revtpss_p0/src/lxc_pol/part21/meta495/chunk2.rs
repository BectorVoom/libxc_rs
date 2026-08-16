//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2090/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2090<F: Float>(t15655: F, t366: F, t3224: F, t4845: F, t127: F, t371: F, t4852: F, t1025: F, t1646: F, t3056: F) -> (F, F, F, F, F) {
    let t15656 = t15655 * t366;
    let t15662 = F::cast_from(0.28582678745379824648e-3_f64) * t3224 * t4845;
    let t15666 = t371 * t127 * t4852;
    let t15668 = F::cast_from(0.28582678745379824648e-3_f64) * t1025 * t15666;
    let t15669 = t1646 * t3056;
    (t15656, t15662, t15666, t15668, t15669)
}
