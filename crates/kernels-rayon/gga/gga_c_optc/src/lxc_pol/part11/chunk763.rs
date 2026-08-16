//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 763/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk763(t3086: f64, t8: f64, t191: f64, t1578: f64, t7274: f64, t1220: f64, t1514: f64, t3101: f64, t2667: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11899 = t8 * t3086;
    let t11900 = t11899 * t191;
    let t11927 = t7274 * t1578;
    let t11928 = t1220 * t11927;
    let t11936 = t3101 * t1514;
    let t11937 = t11936 * t2667;
    (t11899, t11900, t11927, t11928, t11936, t11937)
}
