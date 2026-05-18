//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 758/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk758<F: Float>(t10425: F, t10510: F, t11235: F, t11687: F, t752: F, t1907: F, t5211: F, t1957: F, t1904: F, t5217: F, t5219: F, t5213: F, t5339: F) -> (F, F, F, F) {
    let t11689 = t10425 + t10510 + t11235 + t11687;
    let t11690 = t11689 * t752;
    let t11691 = t5211 * t1907;
    let t11693 = F::new(3.0) * t11691 * t1957;
    let t11694 = t1904 * t5217;
    let t11696 = F::new(6.0) * t11694 * t5219;
    let t11698 = F::new(3.0) * t5213 * t5339;
    (t11690, t11693, t11696, t11698)
}
