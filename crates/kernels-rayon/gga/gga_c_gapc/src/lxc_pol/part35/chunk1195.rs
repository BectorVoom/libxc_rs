//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1195/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1195(t11320: f64, t11496: f64, t628: f64, t11499: f64, t34372: f64, t8621: f64, t1908: f64, t22117: f64, t3699: f64, t5144: f64, t116: f64, t1899: f64, t33666: f64) -> (f64, f64, f64, f64) {
    let t34808 = t628 * t11320 * t11496;
    let t34811 = t628 * t11499 * t11496;
    let t34813 = t34372 * t8621;
    let t34819 = t3699 * t22117 * t1908 * t5144;
    let t34820 = t116 * t1899 * t33666 * t34819;
    (t34808, t34811, t34813, t34820)
}
