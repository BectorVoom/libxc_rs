//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1296/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1296(t1105: f64, t13701: f64, t18899: f64, t18961: f64, t18968: f64, t18970: f64, t18973: f64, t18977: f64, t48510: f64, t48511: f64, t48512: f64, t48513: f64, t48514: f64, t48515: f64, t48516: f64, t49430: f64, t49431: f64, t49432: f64, t804: f64) -> f64 {
    let t50771 = 12.0_f64 * t1105 * t13701 * t804 - t18899 - t18961 - t18968 + t18970 + t18973 - t18977 + t48510 - t48511 + t48512 - t48513 + t48514 - t48515 - t48516 - t49430 + t49431 + t49432;
    t50771
}
