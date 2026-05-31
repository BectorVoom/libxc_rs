//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1390/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1390<F: Float>(t22317: F, t27494: F, t17311: F, t28580: F, t1555: F, t29487: F, t4189: F, t48044: F, t8186: F, t12345: F, t29427: F, t5900: F, t97991: F) -> (F, F, F, F, F, F) {
    let t103900 = F::cast_from(2.0_f64) * t27494 * t22317;
    let t103905 = F::cast_from(4.0_f64) * t17311 * t28580;
    let t103909 = F::cast_from(2.0_f64) * t4189 * t29487 * t1555;
    let t103914 = F::cast_from(4.0_f64) * t48044 * t8186;
    let t103917 = F::cast_from(12.0_f64) * t12345 * t29427 * t1555;
    let t103925 = F::cast_from(4.0_f64) * t97991 * t5900;
    (t103900, t103905, t103909, t103914, t103917, t103925)
}
