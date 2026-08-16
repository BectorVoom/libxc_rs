//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1308/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1308(t12433: f64, t1616: f64, t687: f64, t1615: f64, t3855: f64, t1617: f64, t2011: f64, t3859: f64, t4915: f64, t12329: f64, t3483: f64, t3537: f64) -> (f64, f64, f64, f64, f64) {
    let t38063 = 4.0_f64 * t1616 * t12433 * t687;
    let t38064 = t3855 * t1615;
    let t38066 = 2.0_f64 * t38064 * t1617;
    let t38069 = 6.0_f64 * t4915 * t3859 * t2011;
    let t38070 = t12329 * t2011;
    let t38073 = 24.0_f64 * t4915 * t3483 * t3537;
    (t38063, t38066, t38069, t38070, t38073)
}
