//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 954/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk954(t1036: f64, t2591: f64, t639: f64, t108: f64, t267: f64, t2740: f64, t7068: f64, t995: f64, t1041: f64, t2718: f64, t1028: f64, t2704: f64) -> (f64, f64, f64, f64, f64) {
    let t24784 = t2591 * t1036;
    let t24785 = t639 * t24784;
    let t24835 = t2740 * t108 * t267;
    let t24848 = t7068 * t995;
    let t24980 = t2718 * t1041;
    let t25049 = t2704 * t1028;
    (t24785, t24835, t24848, t24980, t25049)
}
