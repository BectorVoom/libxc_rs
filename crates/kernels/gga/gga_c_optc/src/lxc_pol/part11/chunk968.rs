//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 968/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk968<F: Float>(t27173: F, t27629: F, t25423: F, t3102: F, t321: F, t3695: F, t429: F, t457: F, t27059: F, t466: F, t115: F, t25834: F, t426: F, t3209: F, t1724: F, t9166: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t27630 = t27629 * t27173;
    let t27644 = t3102 * t25423;
    let t27651 = 0.85858385084333410912e-1 * t457 * t321 * t3695 * t429;
    let t27670 = 0.5224665647534064904e-2 * t466 * t27059;
    let t27705 = t426 * t25834 * t115;
    let t27706 = t3209 * t27705;
    let t27712 = t1724 * t27705;
    let t27780 = t9166 * sigma2;
    (t27630, t27644, t27651, t27670, t27705, t27706, t27712, t27780)
}
