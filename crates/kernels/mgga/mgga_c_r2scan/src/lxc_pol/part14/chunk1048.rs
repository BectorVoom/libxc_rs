//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1048/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1048<F: Float>(t10966: F, t1103: F, t269: F, t955: F, t11505: F, t494: F, t97: F, t2330: F, t23791: F, t3446: F, t37475: F, t970: F, t105: F, t2530: F, t797: F, t8296: F) -> (F, F, F, F, F, F) {
    let t40659 = t10966 * t1103 * t955 * t269;
    let t40664 = t97 * t11505 * t494;
    let t40667 = t23791 * t2330;
    let t40672 = t3446 * t37475 * t970;
    let t40681 = t97 * t105 * t2530;
    let t40691 = t797 * t8296;
    (t40659, t40664, t40667, t40672, t40681, t40691)
}
