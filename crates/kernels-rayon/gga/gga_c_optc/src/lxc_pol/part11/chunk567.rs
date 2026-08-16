//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 567/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk567(t3400: f64, t1996: f64, t2000: f64, t2044: f64, t2047: f64, t4560: f64, t4581: f64, t4604: f64, t4606: f64, t4607: f64, t4608: f64, t1761: f64, t1787: f64, t1795: f64, t1799: f64, t1834: f64, t1838: f64, t1873: f64, t1966: f64, t1968: f64, t1985: f64, t1992: f64, t2050: f64) -> (f64, f64) {
    let t4634 = 2.0_f64 * t3400;
    let t4635 = t4604 - t4606 - t4607 - t4608 - t4560 + t4634 + t4581 - t1996 - t2000 + t2044 + t2047;
    let t4636 = t2050 - t1834 + t1992 - t1838 + t1985 + t1873 - t1968 - t1966 - t1761 + t1799 + t1787 + t1795;
    (t4635, t4636)
}
