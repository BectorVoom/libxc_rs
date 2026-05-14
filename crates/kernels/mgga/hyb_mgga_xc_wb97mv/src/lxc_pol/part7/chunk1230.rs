//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1230/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1230<F: Float>(t4126: F, t6715: F, t683: F, t4130: F, t2035: F, t21951: F, t4134: F, t4122: F, t10554: F, t26187: F, t3155: F, t10549: F, t2039: F, t10550: F, t26226: F, t8528: F) -> (F, F, F, F, F, F, F) {
    let t29970 = t683 * t6715 * t4126;
    let t29973 = t683 * t6715 * t4130;
    let t29976 = t2035 * t21951 * t4134;
    let t29985 = t683 * t6715 * t4122;
    let t29989 = t3155 * t26187 * t10554;
    let t29991 = t10549 * t2039;
    let t29996 = t8528 * t26226 * t10550;
    (t29970, t29973, t29976, t29985, t29989, t29991, t29996)
}
