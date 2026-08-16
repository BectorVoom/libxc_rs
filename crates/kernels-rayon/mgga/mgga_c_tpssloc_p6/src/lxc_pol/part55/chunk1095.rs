//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1095/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1095(t1985: f64, t32769: f64, t1375: f64, t2016: f64, t26366: f64, t31115: f64, t31129: f64, t32737: f64, t32758: f64, t32764: f64, t32766: f64, t5215: f64, t5321: f64, t6958: f64, t7729: f64, t8476: f64, t8486: f64) -> f64 {
    let t32771 = 0.16449340668482264365e-1_f64 * t1985 * t32769;
    let t32780 = -t1375 * t32758 + 4.0_f64 * t1375 * t32766 - 2.0_f64 * t2016 * t26366 + 2.0_f64 * t5215 * t8476 - t5215 * t8486 + 2.0_f64 * t5321 * t8476 - t5321 * t8486 + 4.0_f64 * t6958 * t7729 + t31115 + t31129 - t32737 + t32764 - t32771;
    t32780
}
