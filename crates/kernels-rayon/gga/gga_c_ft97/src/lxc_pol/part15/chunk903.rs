//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 903/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk903(t15706: f64, t3020: f64, t1593: f64, t4466: f64, t419: f64, t4487: f64, t626: f64, t4479: f64, t4483: f64, t408: f64, t15712: f64, t1771: f64, t4463: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58407 = t3020 * t15706;
    let t58513 = t1593 * t4466;
    let t58708 = t419 * t626 * t4487;
    let t58719 = t419 * t626 * t4479;
    let t58730 = t419 * t626 * t4483;
    let t58877 = t408 * t4466;
    let t58911 = t3020 * t15712;
    let t58969 = t1771 * t4463;
    (t58407, t58513, t58708, t58719, t58730, t58877, t58911, t58969)
}
