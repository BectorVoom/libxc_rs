//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1005/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1005(t1087: f64, t2188: f64, t2254: f64, t6: f64, t6851: f64, t6172: f64, t7418: f64, t8131: f64, t8141: f64, t967: f64, t2315: f64, t2553: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15835 = t1087 * t2188;
    let t15843 = t1087 * t2254;
    let t15853 = t6851 * t6;
    let t15884 = t7418 * t6172;
    let t15938 = t8131 * t967 * t8141;
    let t16133 = t2553 * t2315;
    (t15835, t15843, t15853, t15884, t15938, t16133)
}
