//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 719/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk719<F: Float>(t3902: F, t5120: F, t91: F, t1154: F, t5092: F, t9890: F, t192: F, t21399: F, t743: F, t20489: F, t738: F, t737: F, t21416: F, t9942: F, t2372: F, t3930: F, t5053: F) -> (F, F, F, F, F, F, F) {
    let t21556 = t91 * t3902 * t5120;
    let t21565 = t5092 * t1154;
    let t21567 = t91 * t9890 * t21565;
    let t21570 = t192 * t743 * t21399;
    let t21572 = t738 * t20489;
    let t21573 = t737 * t21572;
    let t21577 = t192 * t9942 * t21416;
    let t21581 = t2372 * t3930 * t5053;
    (t21556, t21567, t21570, t21572, t21573, t21577, t21581)
}
