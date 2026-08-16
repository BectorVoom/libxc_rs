//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1164/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1164(t1773: f64, t1980: f64, t1982: f64, t1983: f64, t1988: f64, t9691: f64, t1742: f64, t1992: f64, t5: f64, t1891: f64, t7605: f64, t2001: f64, t5690: f64) -> (f64, f64, f64, f64, f64) {
    let t40156 = t1980 * t1982 * t1773 * t1983;
    let t40158 = t1988 * t9691;
    let t40163 = t1980 * t1982 * t5 * t1742 * t1992;
    let t40166 = t7605 * t1891;
    let t40168 = t2001 * t5690;
    (t40156, t40158, t40163, t40166, t40168)
}
