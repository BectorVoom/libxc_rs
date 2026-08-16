//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1267/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1267(t1306: f64, t2993: f64, t30387: f64, t30502: f64, t30704: f64, t30706: f64, t30708: f64, t30710: f64, t30714: f64, t30716: f64, t30718: f64, t30722: f64, t9721: f64) -> f64 {
    let t31004 = 6.0_f64 * t1306 * t2993 * t9721 + t30387 + t30502 + t30704 - t30706 + t30708 + t30710 + t30714 - t30716 + t30718 + t30722;
    t31004
}
