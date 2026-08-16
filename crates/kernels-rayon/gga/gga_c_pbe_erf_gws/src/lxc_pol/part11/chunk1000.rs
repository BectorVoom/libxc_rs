//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1000/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1000(t20962: f64, t3820: f64, t11609: f64, t4395: f64, t6472: f64, t11924: f64, t20550: f64, t3875: f64, t6505: f64, t3857: f64, t6455: f64, t20189: f64, t3116: f64, t3792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38735 = t20962 * t3820;
    let t38761 = t4395 * t11609;
    let t38850 = t6472 * t11609;
    let t38870 = t20550 * t11924;
    let t38979 = t6505 * t3875;
    let t38981 = t6455 * t3857;
    let t38997 = t3116 * t20189 * t3792;
    (t38735, t38761, t38850, t38870, t38979, t38981, t38997)
}
