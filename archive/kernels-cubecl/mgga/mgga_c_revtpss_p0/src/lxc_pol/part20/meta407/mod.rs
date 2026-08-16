//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1506;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1507;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1508;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta407<F: Float>(t11662: F, t11710: F, t4892: F, t3046: F, t3298: F, t4891: F, t1052: F, t11243: F, t11240: F, t3144: F, t11263: F, t3169: F, t11977: F, t3173: F, t12009: F, t12013: F, t11916: F, t11999: F, t3043: F, t3140: F, t3149: F, t11239: F, t989: F, t11629: F, t3160: F, t11874: F, t16048: F, t1042: F, t11252: F, t11634: F, t11862: F, t11877: F, t2251: F, t3075: F, t3127: F, t3157: F, t3164: F, t4801: F, t12046: F, t15905: F, t994: F, t3114: F, t42416: F, t11652: F, t3172: F, t4837: F, t1063: F, t11986: F, t247: F, t2862: F) -> (F, F, F, F, F, F, F, F) {
        let (t42637, t42643, t42646, t42648, t42656) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1506::<F>(t11662, t11710, t4892, t3046, t3298, t4891, t1052, t11243, t11240, t3144, t11263, t3169);
        let (t42658, t42660, t42662, t42664, t42665, t42668, t42669) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1507::<F>(t11977, t3173, t12009, t12013, t11916, t11999, t3043, t3140, t3149, t11239, t989, t11629);
        let t42678 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1508::<F>(t3160, t42664, t11874, t16048, t1042, t11252, t11634, t11862, t11877, t2251, t3075, t3127, t3157, t3164, t42643, t42648, t42656, t42658, t42660, t42662, t42665, t42669, t4801);
        let (t42690, t42695, t42699, t42710) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1509::<F>(t12046, t15905, t994, t3114, t42416, t11652, t3172, t4837, t1063, t11986, t247, t2862);
    (t42637, t42646, t42668, t42678, t42690, t42695, t42699, t42710)
}
