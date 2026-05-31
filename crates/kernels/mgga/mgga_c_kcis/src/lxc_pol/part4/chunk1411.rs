//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1411/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1411<F: Float>(t2128: F, t4500: F, t12930: F, t12933: F, t17307: F, t17313: F, t17315: F, t17319: F, t17322: F, t17328: F, t4475: F, t4480: F, t6222: F, t6225: F, t6256: F) -> F {
    let t18355 = t2128 * t4500;
    let t18364 = -t12930 * t2128 + F::cast_from(4.0_f64) * t12933 * t6225 + F::cast_from(2.0_f64) * t18355 * t4480 - F::cast_from(2.0_f64) * t4475 * t6256 - t4500 * t6222 - t17307 - t17313 + t17315 + t17319 + t17322 - t17328;
    t18364
}
