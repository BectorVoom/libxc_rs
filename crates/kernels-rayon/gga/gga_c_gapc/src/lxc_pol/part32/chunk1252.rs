//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1252/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1252(t25514: f64, t2920: f64, t2974: f64, t8290: f64, t3646: f64, t8493: f64, t11217: f64, t8510: f64, t1492: f64, t3640: f64, t101: f64, t11270: f64) -> (f64, f64, f64, f64, f64) {
    let t35447 = t2920 * t25514 * t2974 * t8290;
    let t35449 = t8493 * t3646;
    let t35451 = t8510 * t11217;
    let t35453 = t1492 * t3640;
    let t35455 = t11270 * t101;
    (t35447, t35449, t35451, t35453, t35455)
}
