//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2127/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2127<F: Float>(t114: F, t22628: F, t655: F, t10201: F, t13448: F, t21818: F, t21827: F, t22590: F, t22593: F, t69: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t22629 = t655 * t22628;
    let t22633 = piecewise3::<F>(t115, F::new(0.0), -t10201 - F::new(11.0) / F::new(3.0) * t13448 - F::new(2.0) * t21818 + t21827 - F::new(3.0) / F::new(4.0) * t69 * t22590 + F::new(3.0) / F::new(4.0) * t69 * t22593 - t69 * t22629 / F::new(8.0));
    (t22629, t22633)
}
