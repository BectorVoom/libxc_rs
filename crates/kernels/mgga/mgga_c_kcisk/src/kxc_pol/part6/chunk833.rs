//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 833/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk833<F: Float>(t742: F, t651: F, t79: F, t747: F, t741: F, t28256: F, t5290: F, t7315: F, t2586: F, t9065: F, t11807: F, t29274: F, t746: F, t2560: F, t9020: F, t2563: F, t9054: F) -> (F, F, F, F, F, F) {
    let t29509 = t742 * t742;
    let t29512 = 1.0 / t651 / t29509 * t79;
    let t29513 = t29512 * t747;
    let t29514 = t741 * t29513;
    let t29516 = t5290 * t28256;
    let t29517 = t7315 * t29516;
    let t29519 = t2586 * t9065;
    let t29520 = t741 * t29519;
    let t29522 = t11807 * t29274;
    let t29523 = t746 * t29522;
    let t29524 = t741 * t29523;
    let t29526 = t2560 * t9020;
    let t29528 = t9054 * t2563;
    (t29514, t29517, t29520, t29524, t29526, t29528)
}
