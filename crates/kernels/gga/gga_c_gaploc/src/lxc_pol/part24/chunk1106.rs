//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1106/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1106<F: Float>(t28126: F, t5841: F, t7810: F, t23296: F, t787: F, t9824: F, t549: F, t6111: F, t7292: F, t15483: F, t2615: F, t9438: F) -> (F, F, F, F, F) {
    let t28810 = F::new(0.38342925953920749676e1) * t7810 * t5841 * t28126;
    let t28811 = t787 * t23296;
    let t28813 = F::new(0.59584149919750711116e-1) * t28811 * t9824;
    let t28816 = F::new(0.23833659967900284446e0) * t6111 * t549 * t7292;
    let t28818 = t2615 * t9438 * t15483;
    (t28810, t28811, t28813, t28816, t28818)
}
