//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 913/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk913<F: Float>(t11881: F, t826: F, t11880: F, t1010: F, t3366: F, t1276: F, t1070: F, t2391: F, t3675: F, t856: F, t11189: F, t11621: F, t3275: F, t11465: F, t3579: F, t11555: F, t3472: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11882 = t11881 * t826;
    let t11883 = t11880 * t11882;
    let t11885 = t3366 * t1010;
    let t11886 = t1276 * t11885;
    let t11888 = t1070 * t2391;
    let t11889 = t1276 * t11888;
    let t11993 = t3675 * t856;
    let t12024 = t11189 * t11621;
    let t12025 = t3275 * t12024;
    let t12026 = 45.0 / 64.0 * t12025;
    let t12027 = t3579 * t11465;
    let t12028 = 5.0 / 16.0 * t12027;
    let t12029 = t3472 * t11555;
    (t11882, t11883, t11885, t11886, t11888, t11889, t11993, t12024, t12025, t12026, t12027, t12028, t12029)
}
