//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 410/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk410(t111: f64, t2010: f64, t1928: f64, t5: f64, t629: f64, t1948: f64, t105: f64, t692: f64, t635: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2011 = t111 * t2010;
    let t2012 = t5 * t1928;
    let t2013 = t629 * t2012;
    let t2017 = t629 * t5 * t1948;
    let t2020 = t105 * t692;
    let t2021 = t2020 * t635;
    (t2011, t2012, t2013, t2017, t2020, t2021)
}
