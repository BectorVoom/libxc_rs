//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1002/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1002<F: Float>(t30294: F, t6175: F, t30153: F, t3953: F, t1312: F, t13878: F, t3952: F, t13895: F, t13894: F, t2105: F, t7802: F) -> (F, F, F, F, F) {
    let t30536 = t6175 * t30294;
    let t30539 = t3953 * t30153;
    let t30540 = t1312 * t30539;
    let t30543 = t13878 * t30153;
    let t30544 = t3952 * t30543;
    let t30547 = t13895 * t30153;
    let t30548 = t13894 * t30547;
    let t30551 = t7802 * t2105;
    (t30536, t30540, t30544, t30548, t30551)
}
