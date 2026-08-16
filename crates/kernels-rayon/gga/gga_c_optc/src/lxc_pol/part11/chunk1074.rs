//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1074/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1074(t1497: f64, t8193: f64, t9114: f64, t15849: f64, t1514: f64, t8113: f64, t9167: f64, t15859: f64, t1170: f64, t1540: f64, t3843: f64, t1150: f64, t1528: f64, t3902: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35730 = t9114 * t1497 * t8193;
    let t35733 = t15849 * t8193;
    let t35745 = t9167 * t1514 * t8113;
    let t35748 = t15859 * t8113;
    let t35825 = t1170 * t3843 * t1540;
    let t35834 = t1150 * t3902 * t1528;
    (t35730, t35733, t35745, t35748, t35825, t35834)
}
