//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1196/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1196<F: Float>(t1719: F, t1809: F, t1660: F, t4741: F, t1665: F, t1647: F, t1883: F, t650: F, t1659: F, t1882: F, t390: F, t631: F, t1664: F, t649: F, t1399: F, t5603: F) -> (F, F, F, F, F, F, F) {
    let t22006 = t1809 * t1719;
    let t22010 = 0.44322962962962962962e0 * t4741 * t1660;
    let t22012 = 0.35640049084583945491e1 * t4741 * t1665;
    let t22023 = 36.0 * t650 * t1883 * t1647;
    let t22030 = 0.42739999999999999999e0 * t390 * t631 * t1882 * t1659;
    let t22034 = 0.34367190188705947438e1 * t390 * t649 * t1882 * t1664;
    let t22036 = 0.56986666666666666664e0 * t1399 * t5603;
    (t22006, t22010, t22012, t22023, t22030, t22034, t22036)
}
