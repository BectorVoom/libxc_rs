//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1002/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1002<F: Float>(t15362: F, t9810: F, t28126: F, t5841: F, t7810: F, t23296: F, t787: F, t9824: F, t549: F, t6111: F, t7292: F, t15483: F, t2615: F, t9438: F, t7416: F, t9830: F) -> (F, F, F, F, F, F, F) {
    let t28800 = 0.11916829983950142223e0 * t15362 * t9810;
    let t28810 = 0.38342925953920749676e1 * t7810 * t5841 * t28126;
    let t28811 = t787 * t23296;
    let t28813 = 0.59584149919750711116e-1 * t28811 * t9824;
    let t28816 = 0.23833659967900284446e0 * t6111 * t549 * t7292;
    let t28818 = t2615 * t9438 * t15483;
    let t28820 = t7416 * t9830;
    (t28800, t28810, t28811, t28813, t28816, t28818, t28820)
}
