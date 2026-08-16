//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1318/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1318(t11772: f64, t11775: f64, t11778: f64, t11780: f64, t11792: f64, t11825: f64, t19995: f64, t19996: f64, t19997: f64, t19998: f64, t6592: f64, t694: f64, t839: f64) -> f64 {
    let t24578 = 3.0_f64 * t6592 * t694 * t839 - t11772 - t11775 + t11778 - t11780 + t11792 + t11825 - t19995 - t19996 + t19997 - t19998;
    t24578
}
