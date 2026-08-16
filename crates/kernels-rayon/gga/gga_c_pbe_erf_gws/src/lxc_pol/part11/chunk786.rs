//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 786/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk786(t3403: f64, t7527: f64, t2612: f64, t3523: f64, t10851: f64, t10872: f64, t10874: f64, t1033: f64, t3392: f64, t10876: f64, t10879: f64, t10500: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12756 = 16.0_f64 / 15.0_f64 * t7527 * t3403;
    let t12758 = 4.0_f64 / 9.0_f64 * t2612 * t3523;
    let t12759 = 16.0_f64 / 45.0_f64 * t10851;
    let t12760 = 4.0_f64 / 15.0_f64 * t10872;
    let t12761 = 8.0_f64 / 15.0_f64 * t10874;
    let t12763 = 4.0_f64 / 5.0_f64 * t1033 * t3392;
    let t12764 = 16.0_f64 / 15.0_f64 * t10876;
    let t12765 = 8.0_f64 / 15.0_f64 * t10879;
    let t12766 = t10500 * t954;
    (t12756, t12758, t12759, t12760, t12761, t12763, t12764, t12765, t12766)
}
