//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1056/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1056<F: Float>(t27705: F, t27780: F, t9114: F, t25560: F, t4463: F, t9115: F, t26261: F, t26264: F, t481: F, t484: F, t9302: F, t27082: F, t496: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27781 = t27780 * t27705;
    let t27786 = t9114 * t27705;
    let t27803 = t4463 * t25560;
    let t27815 = t9115 * t25560;
    let t27866 = F::new(0.20106419753086419753e2) * t26261;
    let t27867 = F::new(0.20068888888888888889e-1) * t26264;
    let t27935 = t481 / t9302 / t484;
    let t27950 = F::new(0.75383950617283950617e4) * t26261;
    let t27951 = F::new(0.12819753086419753086e4) * t26264;
    let t28010 = t27082 * t496;
    (t27781, t27786, t27803, t27815, t27866, t27867, t27935, t27950, t27951, t28010)
}
