//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 953/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk953(t5360: f64, t872: f64, t5379: f64, t868: f64, t14690: f64, t557: f64, t1308: f64, t3909: f64, t12203: f64, t3918: f64, t5385: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15175 = t5360 * t872;
    let t15177 = t868 * t5379;
    let t15179 = t14690 * t557;
    let t15184 = t1308 * t3909;
    let t15190 = t12203 * t5385 * t3918;
    let t15192 = t852 * t5379;
    (t15175, t15177, t15179, t15184, t15190, t15192)
}
