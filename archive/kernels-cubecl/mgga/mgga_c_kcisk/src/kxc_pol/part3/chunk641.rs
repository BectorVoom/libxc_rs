//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 641/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk641<F: Float>(t5283: F, t718: F, t41: F, t719: F, t1646: F, t725: F, t707: F, t4594: F, t702: F, t1797: F, t5061: F, t5320: F) -> (F, F, F, F, F, F, F) {
    let t7315 = t5283 * t718;
    let t7316 = t41 * t719;
    let t7349 = t725 * t1646;
    let t7360 = t725 * t707;
    let t7370 = t4594 * t702;
    let t7378 = t1797 * t702;
    let t7429 = t5061 * t5320;
    (t7315, t7316, t7349, t7360, t7370, t7378, t7429)
}
