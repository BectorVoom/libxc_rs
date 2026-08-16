//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 422/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk422(t1761: f64, t1787: f64, t1795: f64, t1799: f64, t1873: f64, t1876: f64, t1878: f64, t1908: f64, t1964: f64, t1966: f64, t1968: f64, t1970: f64) -> f64 {
    let t2052 = t1873 + t1964 + t1876 - t1878 - t1968 + t1970 + t1966 - t1761 + t1799 + t1908 + t1787 + t1795;
    t2052
}
