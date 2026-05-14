//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1088/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1088<F: Float>(t1065: F, t2526: F, t3270: F, t10667: F, t105: F, t2530: F, t97: F, t10669: F, t10630: F, t11479: F, t3262: F, t3617: F, t5086: F, t10998: F, t3275: F, t797: F, t8296: F) -> (F, F, F, F, F) {
    let t40676 = t1065 * t2526;
    let t40677 = t3270 * t40676;
    let t40679 = 3.0 / 2.0 * t10667 * t40677;
    let t40681 = t97 * t105 * t2530;
    let t40683 = 3.0 / 2.0 * t40681 * t10669;
    let t40686 = 3.0 / 4.0 * t3262 * t11479 * t10630;
    let t40687 = t5086 * t3617;
    let t40690 = 45.0 / 64.0 * t3275 * t40687 * t10998;
    let t40691 = t797 * t8296;
    (t40679, t40683, t40686, t40690, t40691)
}
