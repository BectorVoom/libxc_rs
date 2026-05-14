//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1403/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1403<F: Float>(t1569: F, t8691: F, t20762: F, t33352: F, t538: F, t24840: F, t25582: F, t25585: F, t25605: F, t2572: F, t2598: F, t2612: F, t29919: F, t29932: F, t29934: F, t29938: F, t29941: F, t29944: F, t360: F, t7984: F, t7987: F, t8783: F, t8796: F, t8800: F) -> (F, F) {
    let t34020 = t1569 * t8691;
    let t34035 = t20762 * t538 * t33352;
    let t34042 = 0.26004665220162805689e0 * t2598 * t360 * t2572 * t34020 + 0.7801399566048841707e1 * t24840 * t360 * t8783 * t2612 + 0.13002332610081402845e0 * t7984 * t8796 + 0.39006997830244208535e0 * t7987 * t8800 - t25582 + t25585 + t25605 + 0.83214928704520978208e1 * t29919 + 0.49390868872016336989e-1 * t34035 + 0.20958572791407956061e0 * t29932 - 0.17465477326173296717e-1 * t29934 - 0.17465477326173296717e-1 * t29938 - 0.87816964854445047168e-1 * t29941 + 0.87816964854445047168e-1 * t29944;
    (t34020, t34042)
}
