//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1127/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1127<F: Float>(t10935: F, t2813: F, t3446: F, t3261: F, t498: F, t97: F, t10648: F, t10971: F, t11564: F, t10966: F, t1103: F, t269: F, t955: F) -> (F, F, F, F) {
    let t40603 = t3446 * t10935 * t2813;
    let t40630 = t97 * t3261 * t498;
    let t40642 = t10648 * t10971 * t11564;
    let t40659 = t10966 * t1103 * t955 * t269;
    (t40603, t40630, t40642, t40659)
}
