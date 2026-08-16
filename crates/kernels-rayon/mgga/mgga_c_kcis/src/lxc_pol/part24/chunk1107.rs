//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1107/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1107(t6330: f64, t7789: f64, t1252: f64, t6334: f64, t6326: f64, t3507: f64, t1646: f64, t1851: f64, t26961: f64, t3515: f64, t26960: f64, t27799: f64, t28094: f64, t28905: f64, t28909: f64, t28913: f64, t28917: f64, t29094: f64, t7788: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29103 = t7789 * t6330;
    let t29104 = t1252 * t29103;
    let t29107 = t7789 * t6334;
    let t29108 = t1252 * t29107;
    let t29111 = t7789 * t6326;
    let t29112 = t3507 * t29111;
    let t29115 = t1646 * t1851;
    let t29116 = t26961 * t29115;
    let t29117 = t3515 * t29116;
    let t29120 = -0.69505208333333333334e-3_f64 * t7788 * t29094 + 0.15476481481481481481e-2_f64 * t27799 + 0.30918233506944444444e-4_f64 * t28094 + 0.23214722222222222222e-2_f64 * t28905 + 0.11607361111111111111e-2_f64 * t28909 + 0.19345601851851851852e-2_f64 * t28913 - 0.23214722222222222222e-2_f64 * t28917 + 0.23168402777777777778e-3_f64 * t7788 * t29104 - 0.11584201388888888889e-3_f64 * t7788 * t29108 - 0.15445601851851851852e-3_f64 * t7788 * t29112 + 0.23168402777777777778e-3_f64 * t26960 * t29117;
    (t29103, t29104, t29107, t29108, t29111, t29112, t29116, t29117, t29120)
}
