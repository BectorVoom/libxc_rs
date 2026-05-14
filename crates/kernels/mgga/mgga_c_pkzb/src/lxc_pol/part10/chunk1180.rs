//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1180/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1180<F: Float>(t1444: F, t452: F, t1424: F, t454: F, t34: F, t4794: F, t38: F, t4810: F, t19442: F, t19453: F, t2620: F, t5322: F, t1532: F, t2557: F, t49: F, t4865: F, t7046: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19470 = t1444 * t452;
    let t19520 = t454 * t1424;
    let t19523 = t34 * t4794;
    let t19530 = t38 * t4810;
    let t19545 = 20.0 * t34 * t19442;
    let t19551 = 20.0 * t38 * t19453;
    let t19620 = t2620 * t5322;
    let t19623 = t2557 * t49 * t1532;
    let t19625 = t7046 * t4865;
    (t19470, t19520, t19523, t19530, t19545, t19551, t19620, t19623, t19625)
}
