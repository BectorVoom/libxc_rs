//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 863/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk863(t1804: f64, t1823: f64, t5218: f64, t7514: f64, t1648: f64, t5545: f64, t1403: f64, t1407: f64, t1663: f64, t1821: f64, t587: f64, t1769: f64, t5548: f64) -> (f64, f64, f64, f64, f64) {
    let t16609 = 64.0_f64 / 15.0_f64 * t5218 * t7514 * t1804 * t1823;
    let t16611 = 16.0_f64 / 9.0_f64 * t1648 * t5545;
    let t16613 = t1663 * t1407 * t1403;
    let t16616 = 16.0_f64 / 5.0_f64 * t587 * t1821 * t16613;
    let t16620 = 16.0_f64 / 15.0_f64 * t587 * t5548 * t1769 * t1804;
    (t16609, t16611, t16613, t16616, t16620)
}
