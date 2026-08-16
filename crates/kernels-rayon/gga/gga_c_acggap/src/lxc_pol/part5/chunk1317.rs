//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1317/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1317(t11756: f64, t11770: f64, t1734: f64, t19981: f64, t19982: f64, t19984: f64, t19986: f64, t19988: f64, t19989: f64, t19990: f64, t19991: f64, t5412: f64, t5506: f64, t694: f64, t695: f64, t96: f64) -> f64 {
    let t24571 = 6.0_f64 * t1734 * t5412 * t96 + 6.0_f64 * t5506 * t694 * t695 - t11756 + t11770 + t19981 + t19982 - t19984 + t19986 - t19988 - t19989 - t19990 - t19991;
    t24571
}
