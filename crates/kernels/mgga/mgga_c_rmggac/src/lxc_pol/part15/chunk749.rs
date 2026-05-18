//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 749/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk749<F: Float>(t2185: F, t7716: F, t1004: F, t107: F, t490: F, t1180: F, t673: F, t7472: F, t7487: F, t7757: F, t1326: F, t1330: F) -> (F, F, F, F, F) {
    let t35151 = t7716 * t2185;
    let t35154 = t1004 * t107;
    let t35155 = t490 * t35154;
    let t35190 = t1180 * t673;
    let t35191 = t7472 * t35190;
    let t35204 = t7487 * t7757;
    let t35206 = t1326 * t1330;
    (t35151, t35155, t35191, t35204, t35206)
}
