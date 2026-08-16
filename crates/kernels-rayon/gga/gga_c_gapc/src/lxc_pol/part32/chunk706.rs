//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 706/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk706(t126: f64, t1554: f64, t120: f64, t1134: f64, t991: f64, t2894: f64, t385: f64, t4059: f64, t522: f64, t1006: f64, t1448: f64, t1464: f64) -> (f64, f64, f64, f64, f64) {
    let t8327 = t126 * t1554;
    let t8328 = t120 * t8327;
    let t8330 = t1134 * t991;
    let t8332 = t385 * t2894;
    let t8334 = t4059 * t522;
    let t8335 = t1006 * t8334;
    let t8337 = t1448 * t1464;
    (t8328, t8330, t8332, t8335, t8337)
}
