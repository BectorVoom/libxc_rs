//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1201/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1201(t12079: f64, t3103: f64, t5313: f64, t1170: f64, t18098: f64, t2586: f64, t16024: f64, t4509: f64, t1168: f64, t17885: f64, t871: f64, t17987: f64, t3234: f64, t9189: f64) -> (f64, f64, f64, f64, f64) {
    let t55364 = t3103 * t12079 * t5313;
    let t55390 = t1170 * t2586 * t18098;
    let t55392 = t16024 * t4509;
    let t55396 = t1168 * t17885 * t871;
    let t55425 = t3234 * t9189 * t17987;
    (t55364, t55390, t55392, t55396, t55425)
}
