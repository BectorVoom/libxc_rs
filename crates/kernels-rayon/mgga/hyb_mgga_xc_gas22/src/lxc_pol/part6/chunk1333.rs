//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1333/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1333(t28985: f64, t28996: f64, t29010: f64, t29023: f64, t788: f64, t810: f64, t10718: f64, t787: f64, t811: f64, t10555: f64, t2229: f64, t2233: f64, t4108: f64) -> (f64, f64, f64, f64) {
    let t29028 = 1.0_f64 * t788 * (t28985 + t28996 + t29010 + t29023) * t810;
    let t29029 = t10718 * t787;
    let t29031 = 2.0_f64 * t29029 * t811;
    let t29033 = 1.0_f64 * t10555 * t2229;
    let t29034 = t4108 * t2233;
    (t29028, t29031, t29033, t29034)
}
