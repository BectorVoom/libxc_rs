//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1092/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1092<F: Float>(t7720: F, t9562: F, t27076: F, t3489: F, t34690: F, t421: F, t26754: F, t2822: F, t283: F, t3368: F, t46978: F, t7774: F, t7772: F, t15573: F, t26976: F, t7788: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92730 = t9562 * t7720;
    let t92732 = t27076 * t3489;
    let t92735 = t421 * t34690;
    let t92740 = t2822 * t26754;
    let t92742 = t3368 * t283;
    let t92748 = t46978 * t7774;
    let t92749 = t7772 * t92748;
    let t92751 = t15573 * t26976;
    let t92752 = t7788 * t92751;
    (t92730, t92732, t92735, t92740, t92742, t92748, t92749, t92751, t92752)
}
