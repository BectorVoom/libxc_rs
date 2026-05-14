//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 682/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk682<F: Float>(t672: F, t685: F, t63: F, t1664: F, t1800: F, t649: F, t2029: F, t689: F, t1663: F, t5381: F, t390: F, t188: F, t1890: F, t1647: F, t1893: F, t644: F) -> (F, F, F, F, F, F) {
    let t5747 = 1.0 / t685 / t672;
    let t5748 = t63 * t5747;
    let t5754 = 0.10310157056611784231e2 * t649 * t1800 * t1664;
    let t5755 = t2029 * t689;
    let t5759 = t1663 * t5381;
    let t5761 = 0.85917975471764868594e0 * t390 * t5759;
    let t5762 = t1890 * t188;
    let t5763 = t1893 * t1647;
    let t5764 = t5762 * t5763;
    let t5766 = 0.2763462240212181411e2 * t390 * t5764;
    let t5767 = t649 * t644;
    (t5748, t5754, t5755, t5761, t5766, t5767)
}
