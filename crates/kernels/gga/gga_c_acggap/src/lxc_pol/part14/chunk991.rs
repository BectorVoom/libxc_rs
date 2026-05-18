//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 991/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk991<F: Float>(t7433: F, t8970: F, t1181: F, t22040: F, t604: F, t7493: F, t21118: F, t7351: F, t7426: F, t1165: F, t21955: F, t30806: F) -> (F, F, F, F) {
    let t35092 = t7433 * t8970;
    let t35093 = F::new(0.18868855373762491241e-2) * t35092;
    let t35096 = t7493 * t1181 * t604 * t22040;
    let t35097 = F::new(0.21437009059034868486e-2) * t35096;
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35101 = F::new(0.12862205435420921092e-2) * t35100;
    let t35113 = t30806 * t1165 * t604 * t21955;
    (t35093, t35097, t35101, t35113)
}
