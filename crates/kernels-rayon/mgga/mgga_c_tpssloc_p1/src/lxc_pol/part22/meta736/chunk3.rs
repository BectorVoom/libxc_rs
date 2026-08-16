//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2419/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2419(t291: f64, t68972: f64, t68992: f64, t21100: f64, t4497: f64, t959: f64, t68934: f64, t68936: f64, t68938: f64, t68940: f64, t68943: f64, t68947: f64, t68949: f64, t68951: f64, t68954: f64) -> (f64, f64, f64) {
    let t68995 = 0.621814e-1_f64 * (t68972 + t68992) * t291;
    let t68998 = 0.6233709278045326953e3_f64 * t959 * t21100 * t4497;
    let t68999 = -t68934 - t68936 - t68938 + t68940 + t68943 + t68947 + t68949 - t68951 - t68954 - t68995 - t68998;
    (t68995, t68998, t68999)
}
