//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1198/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1198<F: Float>(t42846: F, t481: F, t35373: F, t795: F, t792: F, t9573: F, t11550: F, t983: F, t10935: F, t3162: F, t3446: F, t3453: F, t9056: F) -> (F, F, F, F, F, F) {
    let t43767 = t42846 * t481;
    let t43771 = t35373 * t795;
    let t43798 = t9573 * t792;
    let t43802 = t11550 * t983;
    let t43820 = t3446 * t10935 * t3162;
    let t43826 = t3446 * t3453 * t9056;
    (t43767, t43771, t43798, t43802, t43820, t43826)
}
