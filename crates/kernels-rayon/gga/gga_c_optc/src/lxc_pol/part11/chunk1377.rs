//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1377/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1377(t11700: f64, t1200: f64, t1565: f64, t16135: f64, t17582: f64, t17585: f64, t17610: f64, t27935: f64, t2886: f64, t36985: f64, t4249: f64, t47331: f64, t485: f64, t53612: f64, t5458: f64, t5469: f64, t58369: f64, t58394: f64, t58433: f64, t58448: f64, t58464: f64, t58470: f64, t58487: f64, t58498: f64, t58511: f64, t58524: f64, t9304: f64) -> f64 {
    let t58528 = (t58369 + t58394 + t58433 + t58448) * t485 - 4.0_f64 * t53612 * t1565 + 12.0_f64 * t47331 * t5458 - 6.0_f64 * t16135 * t5469 - 24.0_f64 * t36985 * t17582 + 24.0_f64 * t11700 * t17585 - 4.0_f64 * t4249 * t17610 + 24.0_f64 * t27935 * t58464 - 36.0_f64 * t9304 * t5458 * t5469 + 6.0_f64 * t2886 * t58470 + 8.0_f64 * t2886 * t1565 * t17610 - t1200 * (t58487 + t58498 + t58511 + t58524);
    t58528
}
