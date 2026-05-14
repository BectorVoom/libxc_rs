//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1002/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1002<F: Float>(t6330: F, t7789: F, t1252: F, t6334: F, t6326: F, t3507: F, t1646: F, t1851: F, t26961: F, t3515: F, t26960: F, t27799: F, t28094: F, t28905: F, t28909: F, t28913: F, t28917: F, t29094: F, t7788: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29103 = t7789 * t6330;
    let t29104 = t1252 * t29103;
    let t29107 = t7789 * t6334;
    let t29108 = t1252 * t29107;
    let t29111 = t7789 * t6326;
    let t29112 = t3507 * t29111;
    let t29115 = t1646 * t1851;
    let t29116 = t26961 * t29115;
    let t29117 = t3515 * t29116;
    let t29120 = -0.69505208333333333334e-3 * t7788 * t29094 + 0.15476481481481481481e-2 * t27799 + 0.30918233506944444444e-4 * t28094 + 0.23214722222222222222e-2 * t28905 + 0.11607361111111111111e-2 * t28909 + 0.19345601851851851852e-2 * t28913 - 0.23214722222222222222e-2 * t28917 + 0.23168402777777777778e-3 * t7788 * t29104 - 0.11584201388888888889e-3 * t7788 * t29108 - 0.15445601851851851852e-3 * t7788 * t29112 + 0.23168402777777777778e-3 * t26960 * t29117;
    (t29103, t29104, t29107, t29108, t29111, t29112, t29116, t29117, t29120)
}
