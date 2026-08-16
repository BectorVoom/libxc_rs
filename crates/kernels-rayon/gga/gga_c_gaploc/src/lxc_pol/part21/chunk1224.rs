//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1224/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1224(t1052: f64, t22139: f64, t23575: f64, t2972: f64, t10805: f64, t5552: f64, t1960: f64, t2728: f64, t3073: f64, t7822: f64, t7332: f64, t8862: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32720 = t22139 * t1052;
    let t32723 = 4.0_f64 * t23575 * t2972;
    let t32731 = 4.0_f64 * t5552 * t10805;
    let t32734 = 4.0_f64 * t1960 * t3073 * t2728;
    let t32736 = 2.0_f64 * t7822 * t3073;
    let t32740 = 2.0_f64 * t8862 * t7332;
    (t32720, t32723, t32731, t32734, t32736, t32740)
}
