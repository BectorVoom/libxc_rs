//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1252/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1252<F: Float>(t3200: F, t3217: F, t3219: F, t1150: F, t1152: F, t3902: F, t1170: F, t2586: F, t9030: F, t115: F, t25834: F, t426: F, t3209: F, t19: F, t27175: F, t1724: F) -> (F, F, F, F, F, F, F) {
    let t27687 = t3217 * t3200 * t3219;
    let t27699 = t1150 * t3902 * t1152;
    let t27702 = t1170 * t2586 * t9030;
    let t27705 = t426 * t25834 * t115;
    let t27706 = t3209 * t27705;
    let t27707 = t27175 * t19;
    let t27712 = t1724 * t27705;
    (t27687, t27699, t27702, t27705, t27706, t27707, t27712)
}
