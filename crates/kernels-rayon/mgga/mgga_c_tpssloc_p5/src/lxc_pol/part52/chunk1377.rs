//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1377/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1377(t191: f64, t192: f64, t27903: f64, t2020: f64, t104977: f64, t1874: f64, t27863: f64, t6525: f64, t116152: f64, t119867: f64, t119869: f64, t119871: f64, t119874: f64, t119875: f64, t123067: f64, t1459: f64, t31880: f64, t32676: f64, t32679: f64, t4037: f64) -> f64 {
    let t123111 = t27903 * t191 * t192;
    let t123112 = t123111 * t2020;
    let t123113 = t104977 * t1874;
    let t123115 = t27863 * t6525;
    let t123117 = -2.0_f64 * t116152 * t1459 - 2.0_f64 * t123067 * t1459 - 2.0_f64 * t31880 * t4037 - t119867 - 2.0_f64 * t119869 - 2.0_f64 * t119871 - t119874 + t119875 + t123112 - 2.0_f64 * t123113 - 2.0_f64 * t123115 - t32676 - t32679;
    t123117
}
