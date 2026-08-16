//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2449;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta658(t3046: f64, t3298: f64, t4891: f64, t11263: f64, t3169: f64, t11977: f64, t3173: f64, t12009: f64, t12013: f64, t11916: f64, t11999: f64, t11874: f64, t16048: f64, t12046: f64, t15905: f64, t994: f64, t3114: f64, t42416: f64, t11652: f64, t3172: f64, t4837: f64, t1063: f64, t11986: f64, t247: f64, t2862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42643, t42656, t42658, t42660, t42662, t42675) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2449(t3046, t3298, t4891, t11263, t3169, t11977, t3173, t12009, t12013, t11916, t11999, t11874, t16048);
        let (t42690, t42695, t42699, t42710) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2450(t12046, t15905, t994, t3114, t42416, t11652, t3172, t4837, t1063, t11986, t247, t2862);
    (t42643, t42656, t42658, t42660, t42662, t42675, t42690, t42695, t42699, t42710)
}
