//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2559/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2559(t11310: f64, t300: f64, t15225: f64, t51811: f64, t51725: f64, t51399: f64, t51401: f64, t51404: f64, t51437: f64, t51439: f64, t51806: f64, t51809: f64, t51814: f64, t51818: f64) -> (f64, f64, f64) {
    let t51819 = t300 * t11310;
    let t51822 = 0.30762056574649219974e4_f64 * t51819 * t15225 * t51811;
    let t51824 = 0.19751673498613801407e-1_f64 * t300 * t51725;
    let t51825 = -t51806 - t51809 + t51399 + t51401 + t51404 - t51814 + t51818 - t51822 + t51824 - t51437 - t51439;
    (t51822, t51824, t51825)
}
