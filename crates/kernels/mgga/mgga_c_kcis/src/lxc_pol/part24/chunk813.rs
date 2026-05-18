//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 813/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk813<F: Float>(t1684: F, t3005: F, t3034: F, t4758: F, t1211: F, t5208: F, t1823: F, t3574: F, t13908: F, t13712: F, t13714: F, t4731: F, t962: F) -> (F, F, F, F, F, F, F, F) {
    let t15304 = t1684 * t3005;
    let t15351 = t4758 * t3034;
    let t15362 = t5208 * t1211;
    let t15369 = t1823 * t3574;
    let t15397 = F::new(0.27785333333333333334e0) * t13908;
    let t15411 = F::new(0.22954444444444444444e0) * t13712;
    let t15432 = F::new(0.2283111111111111111e-1) * t13714;
    let t15445 = t4731 * t962;
    (t15304, t15351, t15362, t15369, t15397, t15411, t15432, t15445)
}
