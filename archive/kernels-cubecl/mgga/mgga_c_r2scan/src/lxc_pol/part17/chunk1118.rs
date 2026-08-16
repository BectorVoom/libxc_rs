//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1118/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1118<F: Float>(t10899: F, t11770: F, t2201: F, t2834: F, t3316: F, t20407: F, t2161: F, t2841: F, t625: F, t37982: F, t7620: F, t10856: F, t7407: F) -> (F, F, F, F, F) {
    let t40220 = t2201 * t10899 * t11770;
    let t40222 = t2834 * t3316;
    let t40228 = t2161 * t20407 * t2841 * t625;
    let t40232 = t37982 * t7620;
    let t40234 = t10856 * t7407;
    (t40220, t40222, t40228, t40232, t40234)
}
