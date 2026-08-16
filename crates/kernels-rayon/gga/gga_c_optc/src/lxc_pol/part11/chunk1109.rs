//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1109/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1109(t11781: f64, t5328: f64, t3200: f64, t3212: f64, t5393: f64, t3137: f64, t3192: f64, t5412: f64, t3217: f64, t5398: f64, t12594: f64, t4492: f64) -> (f64, f64, f64, f64, f64) {
    let t45343 = t11781 * t5328;
    let t45418 = t3212 * t3200 * t5393;
    let t45421 = t3192 * t3137 * t5412;
    let t45424 = t3217 * t3200 * t5398;
    let t45430 = t4492 * t12594;
    (t45343, t45418, t45421, t45424, t45430)
}
