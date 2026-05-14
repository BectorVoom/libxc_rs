//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 957/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk957<F: Float>(t14402: F, t7704: F, t2894: F, t2811: F, t330: F, t1008: F, t1646: F, t4947: F, t26679: F, t4547: F, t283: F, t4981: F, t990: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27815 = t7704 * t14402;
    let t27816 = t2894 * t27815;
    let t27819 = t2811 * t330;
    let t27820 = t1646 * t1008;
    let t27821 = t27819 * t27820;
    let t27822 = t4947 * t27821;
    let t27825 = t26679 * t4547;
    let t27826 = t4947 * t27825;
    let t27832 = t4981 * t283 * t990;
    (t27815, t27816, t27819, t27820, t27821, t27822, t27825, t27826, t27832)
}
