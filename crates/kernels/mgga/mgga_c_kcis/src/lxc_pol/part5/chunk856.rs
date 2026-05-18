//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 856/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk856<F: Float>(t3820: F, t6957: F, t3794: F, t5469: F, t6939: F, t6942: F, t6946: F) -> (F, F) {
    let t6958 = t3820 * t6957;
    let t6964 = t3794 + F::new(2.0) / F::new(9.0) * t5469 - F::new(2.0) / F::new(9.0) * t6939 + F::new(2.0) / F::new(3.0) * t6942 - t6946 / F::new(3.0);
    (t6958, t6964)
}
