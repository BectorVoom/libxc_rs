//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1376/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1376(t31918: f64, t4028: f64, t26114: f64, t8675: f64, t26179: f64, t31908: f64, t7458: f64, t113: f64, t119792: f64, t119858: f64, t119862: f64, t123044: f64, t123074: f64, t123088: f64, t1774: f64, t1976: f64, t27371: f64, t31877: f64, t31880: f64, t32674: f64, t4073: f64, t5107: f64, t574: f64, t8667: f64) -> f64 {
    let t123091 = t4028 * t31918;
    let t123093 = t26114 * t8675;
    let t123095 = t26179 * t8675;
    let t123097 = t7458 * t31908;
    let t123101 = t119858 - t113 * (t123044 + t119792) - t31877 * t1774 - t8667 * t5107 - t27371 * t1976 + (t123074 + t123088) * t574 - 2.0_f64 * t123091 - 2.0_f64 * t123093 - 2.0_f64 * t123095 - 2.0_f64 * t123097 - 2.0_f64 * t31880 * t4073 - t119862 - t32674;
    t123101
}
