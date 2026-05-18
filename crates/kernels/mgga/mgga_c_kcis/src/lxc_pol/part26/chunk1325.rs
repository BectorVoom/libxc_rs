//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1325/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1325<F: Float>(t1394: F, t27364: F, t6904: F, t22271: F, t5780: F, t7923: F, t20975: F, t27387: F, t20980: F, t20985: F, t21894: F, t1014: F, t29340: F) -> (F, F, F, F, F, F, F) {
    let t102698 = t1394 * t27364 * t6904;
    let t102701 = t5780 * t7923 * t22271;
    let t102706 = t1394 * t27387 * t20975;
    let t102709 = t1394 * t7923 * t20980;
    let t102712 = t1394 * t7923 * t20985;
    let t102715 = t5780 * t7923 * t21894;
    let t102723 = t1014 * t29340;
    (t102698, t102701, t102706, t102709, t102712, t102715, t102723)
}
