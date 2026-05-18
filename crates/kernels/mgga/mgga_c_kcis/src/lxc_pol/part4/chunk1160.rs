//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1160/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1160<F: Float>(t1176: F, t5165: F, t13265: F, t3438: F, t5175: F, t1121: F, t4823: F, t13105: F, t381: F, t1189: F, t1809: F, t3355: F) -> (F, F, F, F, F, F) {
    let t14712 = t5165 * t1176;
    let t14714 = t3438 * t13265;
    let t14715 = t5175 * t14714;
    let t14717 = t4823 * t1121;
    let t14718 = t3438 * t14717;
    let t14719 = t5175 * t14718;
    let t14721 = t13105 * t381;
    let t14722 = t14721 * t1189;
    let t14724 = t1809 * t3355;
    (t14712, t14715, t14717, t14719, t14722, t14724)
}
