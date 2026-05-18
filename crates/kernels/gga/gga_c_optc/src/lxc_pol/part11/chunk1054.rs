//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1054/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1054<F: Float>(t25423: F, t9168: F, t1027: F, t19: F, t1113: F, t530: F, t4434: F, t3181: F, t442: F, t462: F, t27173: F, t3102: F) -> (F, F, F, F, F, F) {
    let t27438 = t9168 * t25423;
    let t27441 = t19 * t1027;
    let t27515 = t530 * t1113;
    let t27552 = t4434 * t25423;
    let t27629 = F::new(1.0) / t3181 / t462 * t442;
    let t27630 = t27629 * t27173;
    let t27644 = t3102 * t25423;
    (t27438, t27441, t27515, t27552, t27630, t27644)
}
