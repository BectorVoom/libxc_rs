//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 430/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk430(t1983: f64, t2020: f64, t1401: f64, t1873: f64, t63: f64, t67: f64) -> (f64, f64, f64) {
    let t2021 = t1983 * t2020;
    let t2028 = 0.135e2_f64 * t1401 * t1873;
    let t2031 = t63 * t67;
    (t2021, t2028, t2031)
}
