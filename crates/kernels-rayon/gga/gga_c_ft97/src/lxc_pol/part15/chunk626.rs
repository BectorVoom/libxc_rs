//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 626/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk626(t14715: f64, t14895: f64, t10580: f64, t2: f64, t1232: f64, t1771: f64, t1228: f64, t8282: f64, t2347: f64, t852: f64, t2360: f64, t1212: f64, t2781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14946 = 4.0_f64 / 27.0_f64 * t14715;
    let t14949 = 4.0_f64 / 9.0_f64 * t14895;
    let t14961 = t10580 * t2;
    let t15011 = t1771 * t1232;
    let t15025 = t8282 * t1228;
    let t15042 = t852 * t2347;
    let t15047 = t852 * t2360;
    let t15051 = t2781 * t1212;
    (t14946, t14949, t14961, t15011, t15025, t15042, t15047, t15051)
}
