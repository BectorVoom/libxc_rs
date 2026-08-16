//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1266/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1266(t1880: f64, t1913: f64, t1941: f64, t19824: f64, t3032: f64, t3827: f64, t3844: f64, t3847: f64, t3849: f64, t3851: f64, t3853: f64, t51: f64, t54: f64, t584: f64, t60: f64, t6004: f64, t63: f64, t66: f64, t69: f64, t7984: f64, t9915: f64, t9934: f64, t9937: f64, t9942: f64, t9945: f64) -> f64 {
    let t27222 = -19.0_f64 / 412876800.0_f64 * t19824 * t3827 * t1880 + t6004 * t3844 * t1880 / 412876800.0_f64 + 10.0_f64 / 3.0_f64 * t3847 * t1880 - 2.0_f64 / 3.0_f64 * t3849 * t1880 - 7.0_f64 / 8.0_f64 * t3851 * t1880 + t3853 * t1880 / 8.0_f64 + t9934 * t1913 / 6.0_f64 + t9937 * t1913 / 8.0_f64 - t3032 * t7984 / 24.0_f64 - t9942 * t1913 / 48.0_f64 - t60 * t9915 * t584 / 5760.0_f64 + t63 * t9915 * t584 / 129024.0_f64 - t66 * t9915 * t584 / 3440640.0_f64 + t69 * t9915 * t584 / 0.10616832e9_f64 - t1941 * t9915 * t584 / 0.37158912e10_f64 + t51 * t9915 * t584 / 3.0_f64 - t54 * t9915 * t584 / 24.0_f64 - t9945 * t1913 / 80.0_f64;
    t27222
}
