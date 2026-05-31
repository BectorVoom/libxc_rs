//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 622/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk622<F: Float>(t3820: F, t6957: F, t3794: F, t5469: F, t6939: F, t6942: F, t6946: F) -> (F, F) {
    let t6958 = t3820 * t6957;
    let t6964 = t3794 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5469 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t6939 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6942 - t6946 / F::cast_from(3.0_f64);
    (t6958, t6964)
}
