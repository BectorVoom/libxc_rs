//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1237/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1237<F: Float>(t11336: F, t37327: F, t39268: F, t3275: F, t3472: F, t40609: F, t10940: F, t12203: F, t41329: F, t41332: F, t41335: F, t41339: F, t41342: F, t41346: F, t41350: F, t41786: F, t41788: F, t41790: F, t41794: F, t41797: F, t41800: F) -> (F, F, F, F) {
    let t41803 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t37327 * t11336 * t39268;
    let t41806 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t3472 * t40609;
    let t41808 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t10940 * t12203;
    let t41809 = -t41329 + t41332 + t41335 + t41339 + t41342 + t41346 - t41350 + t41786 + t41788 + t41790 + t41794 + t41797 - t41800 + t41803 - t41806 - t41808;
    (t41803, t41806, t41808, t41809)
}
