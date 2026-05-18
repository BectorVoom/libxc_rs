//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1223/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1223<F: Float>(t10669: F, t40681: F, t10630: F, t11479: F, t3262: F, t3617: F, t5086: F, t10998: F, t3275: F, t797: F, t8296: F, t3276: F) -> (F, F, F, F) {
    let t40683 = F::new(3.0) / F::new(2.0) * t40681 * t10669;
    let t40686 = F::new(3.0) / F::new(4.0) * t3262 * t11479 * t10630;
    let t40687 = t5086 * t3617;
    let t40690 = F::new(45.0) / F::new(64.0) * t3275 * t40687 * t10998;
    let t40691 = t797 * t8296;
    let t40694 = F::new(5.0) / F::new(16.0) * t3275 * t3276 * t40691;
    (t40683, t40686, t40690, t40694)
}
