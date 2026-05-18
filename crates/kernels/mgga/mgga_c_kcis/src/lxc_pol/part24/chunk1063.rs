//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1063/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1063<F: Float>(t27957: F, t4947: F, t3489: F, t4981: F, t7699: F, t8030: F, t1014: F, t8054: F, t5019: F, t7726: F, t303: F, t15573: F, t8041: F) -> (F, F, F, F, F, F, F) {
    let t27958 = t4947 * t27957;
    let t27964 = t4981 * t3489;
    let t27967 = t8030 * t7699;
    let t27969 = t1014 * t8054;
    let t27971 = t7726 * t5019;
    let t27972 = t303 * t27971;
    let t27974 = t15573 * t8041;
    (t27958, t27964, t27967, t27969, t27971, t27972, t27974)
}
