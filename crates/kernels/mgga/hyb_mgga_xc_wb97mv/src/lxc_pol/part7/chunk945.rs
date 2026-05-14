//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 945/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk945<F: Float>(t795: F, t8966: F, t238: F, t3361: F, t800: F, t3365: F, t1323: F, t2187: F, t242: F, t3326: F, t778: F, t226: F, t8927: F, t2205: F, t3346: F, t790: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8969 = t795 * t8966;
    let t8972 = t238 * t800 * t3361;
    let t8973 = 0.33114e0 * t8972;
    let t8975 = t238 * t800 * t3365;
    let t8976 = 0.33114e0 * t8975;
    let t8977 = t2187 * t1323;
    let t8979 = t238 * t242 * t8977;
    let t8981 = t778 * t3326;
    let t8983 = t238 * t242 * t8981;
    let t8985 = t226 * t8927;
    let t8987 = t238 * t242 * t8985;
    let t8989 = t2205 * t3346;
    let t8990 = t8989 * t790;
    (t8969, t8972, t8973, t8975, t8976, t8977, t8979, t8981, t8983, t8985, t8987, t8990)
}
