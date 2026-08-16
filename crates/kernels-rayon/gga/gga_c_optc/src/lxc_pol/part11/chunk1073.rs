//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1073/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1073(t1544: f64, t7448: f64, t1516: f64, t8529: f64, t11936: f64, t1519: f64, t3843: f64, t1133: f64, t1499: f64, t8546: f64, t8113: f64, t15855: f64, t8193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35441 = t1544 * t7448;
    let t35453 = t1516 * t8529;
    let t35559 = t11936 * t7448;
    let t35576 = t3843 * t1519;
    let t35577 = t1133 * t35576;
    let t35643 = t1499 * t8546;
    let t35653 = t11936 * t8113;
    let t35724 = t15855 * t8193;
    (t35441, t35453, t35559, t35576, t35577, t35643, t35653, t35724)
}
