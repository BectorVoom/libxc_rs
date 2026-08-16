//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2235/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2235(t17884: f64, t3048: f64, t1009: f64, t17875: f64, t1011: f64, t1019: f64, t3030: f64, t5848: f64, t3032: f64, t3129: f64, t3038: f64, t10891: f64, t17655: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61715 = t3048 * t17884;
    let t61729 = t17875 * t1009;
    let t61731 = t61729 * t1011 * t1019;
    let t61734 = t5848 * t3030;
    let t61735 = t61734 * t3032;
    let t61736 = t61735 * t3129;
    let t61739 = t61735 * t3038;
    let t61742 = t10891 * t17655;
    (t61715, t61729, t61731, t61734, t61736, t61739, t61742)
}
