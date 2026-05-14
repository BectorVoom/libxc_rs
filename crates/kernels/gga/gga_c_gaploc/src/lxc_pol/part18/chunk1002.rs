//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1002/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1002<F: Float>(t23296: F, t787: F, t9824: F, t549: F, t6111: F, t7292: F, t15483: F, t2615: F, t9438: F, t7416: F, t9830: F, t10029: F, t2464: F, t2465: F, t2684: F, t7258: F) -> (F, F, F, F, F, F, F) {
    let t28811 = t787 * t23296;
    let t28813 = 0.59584149919750711116e-1 * t28811 * t9824;
    let t28816 = 0.23833659967900284446e0 * t6111 * t549 * t7292;
    let t28818 = t2615 * t9438 * t15483;
    let t28820 = t7416 * t9830;
    let t28821 = 0.76685851907841499352e0 * t28820;
    let t28822 = t7416 * t10029;
    let t28823 = 0.1022478025437886658e1 * t28822;
    let t28827 = 0.17041300423964777634e0 * t2684 * t2464 * t2465 * t7258;
    (t28811, t28813, t28816, t28818, t28821, t28823, t28827)
}
