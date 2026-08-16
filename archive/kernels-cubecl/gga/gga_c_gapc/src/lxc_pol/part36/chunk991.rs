//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 991/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk991<F: Float>(t11397: F, t277: F, t332: F, t7877: F, t959: F, t11399: F, t2547: F, t3788: F, t3784: F, t11311: F, t2619: F, t1086: F, t6182: F) -> (F, F, F, F, F, F, F, F) {
    let t11977 = t277 * t11397;
    let t11979 = t7877 * t959 * t332;
    let t11980 = t11399 * t11979;
    let t11981 = t11977 * t11980;
    let t11983 = t2547 * t3788;
    let t11984 = t3784 * t11983;
    let t11986 = t2619 * t11311;
    let t11987 = t1086 * t6182;
    (t11977, t11979, t11980, t11981, t11983, t11984, t11986, t11987)
}
