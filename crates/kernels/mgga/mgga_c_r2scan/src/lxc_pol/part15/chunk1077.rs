//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1077/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1077<F: Float>(t1576: F, t546: F, t2079: F, t545: F, t25851: F, t512: F, t6156: F, t10757: F, t776: F, t261: F, t6499: F, t7614: F) -> (F, F, F, F, F, F) {
    let t37965 = t546 * t1576;
    let t37972 = t545 * t2079;
    let t37982 = t512 * t25851;
    let t37983 = t37982 * t6156;
    let t37985 = t776 * t10757;
    let t37988 = t7614 * t261 * t6499;
    (t37965, t37972, t37982, t37983, t37985, t37988)
}
