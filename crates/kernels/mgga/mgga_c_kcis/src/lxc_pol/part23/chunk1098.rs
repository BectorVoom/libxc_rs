//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1098/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1098<F: Float>(t15870: F, t5661: F, t98034: F, t15874: F, t4160: F, t94425: F, t1464: F, t1489: F, t28503: F, t6037: F, t1394: F, t27379: F, t28499: F, t27383: F, t4153: F, t28356: F) -> (F, F, F, F, F, F) {
    let t98036 = t5661 * t98034 * t15870;
    let t98039 = t4160 * t94425 * t15874;
    let t98043 = t1464 * t28503 * t6037 * t1489;
    let t98046 = t1394 * t28499 * t27379;
    let t98049 = t4153 * t28499 * t27383;
    let t98052 = t1394 * t28356 * t27379;
    (t98036, t98039, t98043, t98046, t98049, t98052)
}
