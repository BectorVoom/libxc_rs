//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1156/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1156<F: Float>(t11506: F, t39324: F, t12574: F, t481: F, t10997: F, t3262: F, t40677: F, t3579: F, t39332: F, t1065: F, t2892: F, t3270: F) -> (F, F, F, F, F) {
    let t42818 = F::new(3.0) / F::new(2.0) * t11506 * t39324;
    let t42819 = t12574 * t481;
    let t42822 = F::new(135.0) / F::new(64.0) * t3262 * t10997 * t42819;
    let t42824 = F::new(3.0) / F::new(2.0) * t11506 * t40677;
    let t42826 = F::new(5.0) / F::new(8.0) * t3579 * t39332;
    let t42829 = t1065 * t2892;
    let t42830 = t3270 * t42829;
    (t42818, t42822, t42824, t42826, t42830)
}
