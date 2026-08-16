//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1506;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1507;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1508;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1509;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta407(t11662: f64, t11710: f64, t4892: f64, t3046: f64, t3298: f64, t4891: f64, t1052: f64, t11243: f64, t11240: f64, t3144: f64, t11263: f64, t3169: f64, t11977: f64, t3173: f64, t12009: f64, t12013: f64, t11916: f64, t11999: f64, t3043: f64, t3140: f64, t3149: f64, t11239: f64, t989: f64, t11629: f64, t3160: f64, t11874: f64, t16048: f64, t1042: f64, t11252: f64, t11634: f64, t11862: f64, t11877: f64, t2251: f64, t3075: f64, t3127: f64, t3157: f64, t3164: f64, t4801: f64, t12046: f64, t15905: f64, t994: f64, t3114: f64, t42416: f64, t11652: f64, t3172: f64, t4837: f64, t1063: f64, t11986: f64, t247: f64, t2862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42637, t42643, t42646, t42648, t42656) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1506(t11662, t11710, t4892, t3046, t3298, t4891, t1052, t11243, t11240, t3144, t11263, t3169);
        let (t42658, t42660, t42662, t42664, t42665, t42668, t42669) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1507(t11977, t3173, t12009, t12013, t11916, t11999, t3043, t3140, t3149, t11239, t989, t11629);
        let t42678 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1508(t3160, t42664, t11874, t16048, t1042, t11252, t11634, t11862, t11877, t2251, t3075, t3127, t3157, t3164, t42643, t42648, t42656, t42658, t42660, t42662, t42665, t42669, t4801);
        let (t42690, t42695, t42699, t42710) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1509(t12046, t15905, t994, t3114, t42416, t11652, t3172, t4837, t1063, t11986, t247, t2862);
    (t42637, t42646, t42668, t42678, t42690, t42695, t42699, t42710)
}
