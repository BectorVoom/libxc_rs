//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1226/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1226<F: Float>(t3104: F, t8905: F, t1111: F, t3088: F, t530: F, t24: F, t8533: F, t310: F, t3648: F, t449: F, t448: F, t123: F, t3108: F, t2849: F, t3117: F, t429: F, t745: F) -> (F, F, F, F, F, F, F, F) {
    let t27048 = t3104 * t8905;
    let t27053 = t1111 * t530 * t3088;
    let t27056 = t1111 * t24 * t8533;
    let t27059 = t310 * t3648 * t449;
    let t27061 = 0.18781521737197933637e-2 * t448 * t27059;
    let t27063 = t3108 * t123 * t8905;
    let t27067 = t3117 * t2849;
    let t27071 = t745 * t429;
    (t27048, t27053, t27056, t27059, t27061, t27063, t27067, t27071)
}
