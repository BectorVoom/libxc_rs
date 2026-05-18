//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 691/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk691<F: Float>(t7754: F, t8072: F, t389: F, t4999: F, t1096: F, t1813: F, t1021: F, t1817: F, t8067: F, t8070: F) -> (F, F, F, F, F) {
    let t8073 = t7754 * t8072;
    let t8075 = t4999 * t389;
    let t8077 = t1096 * t1813;
    let t8079 = t1021 * t1817;
    let t8081 = t8067 / F::new(16.0) - t8070 / F::new(16.0) + t8073 / F::new(24.0) - t8075 / F::new(128.0) + t8077 / F::new(128.0) - t8079 / F::new(96.0);
    (t8073, t8075, t8077, t8079, t8081)
}
