//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 856/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk856(t2896: f64, t43: f64, t47: f64, t2908: f64, t50: f64, t52: f64, t3901: f64, t872: f64, t3909: f64, t852: f64, t180: f64, t3645: f64) -> (f64, f64, f64, f64, f64) {
    let t12161 = 1.0_f64 / t47 / t2896 / t43;
    let t12177 = 1.0_f64 / t52 / t2908 / t50;
    let t12196 = t3901 * t872;
    let t12198 = t852 * t3909;
    let t12200 = t3645 * t180;
    (t12161, t12177, t12196, t12198, t12200)
}
