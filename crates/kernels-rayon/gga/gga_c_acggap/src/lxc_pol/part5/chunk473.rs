//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 473/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk473(t150: f64, t1825: f64, t1826: f64, t1713: f64, t921: f64, t1734: f64, t402: f64, t153: f64, t155: f64, t519: f64, t521: f64) -> (f64, f64, f64, f64) {
    let t1828 = (t1825 + t1826) * t150;
    let t1832 = t921 * t1713;
    let t1835 = t402 * t1734;
    let t1838 = -12.0_f64 * t153 * t1832 + 3.0_f64 * t153 * t1835 - t155 * t1828 + 6.0_f64 * t519 * t521;
    (t1828, t1832, t1835, t1838)
}
