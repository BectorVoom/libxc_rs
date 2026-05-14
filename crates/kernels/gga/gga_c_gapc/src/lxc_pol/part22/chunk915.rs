//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 915/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk915<F: Float>(t11970: F, t2761: F, t11969: F, t1026: F, t761: F, t1093: F, t11397: F, t277: F, t332: F, t7877: F, t959: F, t11399: F, t2547: F, t3788: F, t3784: F, t11311: F, t2619: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11971 = t2761 * t11970;
    let t11972 = t11969 * t11971;
    let t11974 = t761 * t1026;
    let t11975 = t11974 * t1093;
    let t11977 = t277 * t11397;
    let t11979 = t7877 * t959 * t332;
    let t11980 = t11399 * t11979;
    let t11981 = t11977 * t11980;
    let t11983 = t2547 * t3788;
    let t11984 = t3784 * t11983;
    let t11986 = t2619 * t11311;
    (t11971, t11972, t11974, t11975, t11977, t11979, t11980, t11981, t11983, t11984, t11986)
}
